// This file is part of the serde-bindgen-core libraries
// Copyright (C) 2022  Altronix Corp. <thomas.chiantia@gmail.com>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// @author Thomas Chiantia <thomas.chiantia@gmail.com>
// @date 2022

//! TODO make trait "FieldType".
//!      A "FieldType" can
//!      - represent itself in context of assignment (default impl)
//!      - represent itself in context of copy (from impl)
//!      - TODO represent itself in context of partial copy
//!      - can tokanize itself as a standard version
//!      - can tokenize itself as an "owned" version
//!      - TODO can tokenize itself as a "partial"

// syn::
use syn::parse::{Error, Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::token::Bracket;
use syn::Ident;
use syn::LitInt;
use syn::Token;

// quote::
use quote::quote;
use quote::ToTokens;

use proc_macro2::TokenStream;

// super:
use super::attributes::{Attribute, DefaultLit};
use super::path::PathNamed;
use super::utils;

#[derive(Clone)]
pub struct Field {
    pub ident: Ident,
    pub tok_vis: Option<Token![pub]>,
    pub tok_col: Token![:],
    pub ty: FieldType,
    pub attributes: Vec<Attribute>,
}

impl Field {
    pub fn stackify(&mut self) {
        self.ty.stackify(&self.attributes);
    }

    pub fn assignment_tokens(&self) -> TokenStream {
        let name = &self.ident;
        let init = self.attributes.iter().find_map(|attr| attr.default());
        let assignment = self.ty.assignment_tokens(&init);
        quote! {#name: #assignment}
    }

    pub fn from_tokens(&self, var: &Ident) -> TokenStream {
        let name = &self.ident;
        let expr = quote! {#var.#name};
        let assignment = self.ty.from_tokens(&syn::parse_quote! {#expr});
        quote! {#name: #assignment}
    }
}

impl Parse for Field {
    fn parse(mut input: ParseStream) -> Result<Self> {
        let mut attributes = Vec::new();
        while input.peek(Token![#]) {
            attributes.push(input.parse::<Attribute>()?);
        }
        Ok(Field {
            tok_vis: utils::maybe(Token![pub], &mut input)?,
            ident: input.parse()?,
            tok_col: input.parse()?,
            ty: input.parse()?,
            attributes,
        })
    }
}

impl ToTokens for Field {
    fn to_tokens(&self, toks: &mut TokenStream) {
        // We consume all the /// data: attributes but leave other doc comments
        self.attributes
            .iter()
            .filter_map(|attr| attr.ignore())
            .for_each(|meta| meta.to_tokens(toks));

        if let Some(vis) = self.tok_vis {
            vis.to_tokens(toks);
        }
        self.ident.to_tokens(toks);
        self.tok_col.to_tokens(toks);
        self.ty.to_tokens(toks);
    }
}

#[derive(Clone)]
#[cfg_attr(feature = "testing", derive(PartialEq, Debug))]
pub enum FieldType {
    RefStr(FieldTypeRef),
    Struct(PathNamed),
    Primative(Ident),
    Array(FieldTypeArray),
}

impl FieldType {
    pub fn stackify(&mut self, attributes: &Vec<Attribute>) {
        use FieldType::*;
        match self {
            RefStr(FieldTypeRef { ident, .. }) => {
                let span = ident.span().clone();
                let len: LitInt = attributes
                    .iter()
                    .find_map(|attr| attr.len())
                    .map(|len| len.clone())
                    .unwrap_or_else(|| LitInt::new("0", span.clone()));
                *self = FieldType::Array(syn::parse_quote! {[u8;#len]});
            }
            Array(FieldTypeArray { ty, .. }) => {
                ty.stackify(attributes);
            }
            Struct(p) => {
                p.stackify();
            }

            _ => {}
        }
    }

    pub fn assignment_tokens(&self, expr: &Option<&DefaultLit>) -> TokenStream {
        match (expr, &self) {
            (Some(e), FieldType::RefStr(_)) => {
                quote! {serde_bindgen_core::SafeCopy::safe_copy(&#e)}
            }
            (None, FieldType::RefStr(_)) => quote! {serde_bindgen_core::SafeCopy::safe_copy(&"")},
            (_, FieldType::Struct(_)) => quote! {Default::default()},
            (None, FieldType::Primative(i)) if i == "bool" => quote! {false},
            (None, FieldType::Primative(_)) => quote! {0},
            (Some(expr), FieldType::Primative(_)) => quote! {#expr},
            (Some(expr), FieldType::Array(_)) if expr.is_array() => quote! {#expr},
            (_, FieldType::Array(a)) => a.surround(|_| a.ty.assignment_tokens(&expr)),
        }
    }

    pub fn from_tokens(&self, expr: &TokenStream) -> TokenStream {
        match &self {
            FieldType::RefStr(_) => quote! {serde_bindgen_core::SafeCopy::safe_copy(&#expr)},
            FieldType::Struct(_) => quote! {From::from(&#expr)},
            FieldType::Primative(_) => quote! {#expr},
            FieldType::Array(a) => a.surround(|n| {
                let i = LitInt::new(&n.to_string(), a.n.span()).token();
                let expr = quote! {#expr[#i]};
                a.ty.from_tokens(&expr)
            }),
        }
    }
}

impl Parse for FieldType {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(Token![&]) && input.peek2(syn::Lifetime) {
            Ok(FieldType::RefStr(input.parse()?))
        } else if input.peek(Token![::]) || input.peek(Ident) && input.peek2(Token![::]) {
            Ok(FieldType::Struct(input.parse()?))
        } else if input.peek(Ident) {
            let path: PathNamed = input.parse()?;
            if path.is_primative() {
                Ok(FieldType::Primative(path.ident))
            } else {
                Ok(FieldType::Struct(path))
            }
        } else if input.peek(Bracket) {
            Ok(FieldType::Array(input.parse()?))
        } else {
            let err = Error::new(input.span(), "Unsupported field type");
            Err(err)
        }
    }
}

impl ToTokens for FieldType {
    fn to_tokens(&self, toks: &mut TokenStream) {
        match self {
            FieldType::RefStr(t) => t.to_tokens(toks),
            FieldType::Struct(t) => t.to_tokens(toks),
            FieldType::Primative(t) => t.to_tokens(toks),
            FieldType::Array(t) => t.to_tokens(toks),
        }
    }
}

#[derive(Clone)]
#[cfg_attr(feature = "testing", derive(PartialEq, Debug))]
pub struct FieldTypeRef {
    pub amp: Token![&],
    pub lifetime: syn::Lifetime,
    pub ident: Ident,
}
impl Parse for FieldTypeRef {
    fn parse(input: ParseStream) -> Result<Self> {
        let amp: Token![&] = input.parse()?;
        let lifetime: syn::Lifetime = input.parse()?;
        let ident = input.parse()?;
        if ident == "str" {
            Ok(FieldTypeRef {
                amp,
                lifetime,
                ident,
            })
        } else {
            let err = Error::new(input.span(), "Can only have str reference");
            Err(err)
        }
    }
}

impl ToTokens for FieldTypeRef {
    fn to_tokens(&self, toks: &mut TokenStream) {
        self.amp.to_tokens(toks);
        self.lifetime.to_tokens(toks);
        self.ident.to_tokens(toks);
    }
}

#[derive(Clone)]
#[cfg_attr(feature = "testing", derive(PartialEq, Debug))]
pub struct FieldTypeArray {
    bracket: Bracket,
    ty: Box<FieldType>,
    semi: Token![;],
    n: LitInt,
}

impl FieldTypeArray {
    pub fn surround<F: FnMut(usize) -> TokenStream>(&self, mut func: F) -> TokenStream {
        let n = self.n.base10_digits().parse().unwrap_or(0);
        let mut toks = TokenStream::new();
        let _bracket = self.bracket.surround(&mut toks, |toks| {
            let mut punc = Punctuated::<TokenStream, Token![,]>::new();
            for i in 0..n {
                punc.push(func(i))
            }
            punc.to_tokens(toks);
        });
        toks
    }
}

impl Parse for FieldTypeArray {
    fn parse(input: ParseStream) -> Result<Self> {
        let inner;
        Ok(FieldTypeArray {
            bracket: syn::bracketed!(inner in input),
            ty: inner.parse()?,
            semi: inner.parse()?,
            n: inner.parse()?,
        })
    }
}

impl ToTokens for FieldTypeArray {
    fn to_tokens(&self, toks: &mut TokenStream) {
        self.bracket.surround(toks, |toks| {
            self.ty.to_tokens(toks);
            self.semi.to_tokens(toks);
            self.n.to_tokens(toks);
        });
    }
}
