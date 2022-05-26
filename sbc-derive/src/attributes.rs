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

use proc_macro2::TokenStream;
use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::token::Bracket;
use syn::ExprArray;
use syn::Ident;
use syn::LitInt;
use syn::Token;

use serde::Deserialize; // TODO feature guard

use quote::ToTokens;

use super::keyword;

#[derive(Clone)]
#[cfg_attr(feature = "testing", derive(Debug))]
pub struct ContainerAttribute {
    pub ident: Ident,
    pub eq: Token![=],
    pub val: syn::LitStr,
}

impl Parse for ContainerAttribute {
    fn parse(input: ParseStream) -> Result<ContainerAttribute> {
        Ok(ContainerAttribute {
            ident: input.parse()?,
            eq: input.parse()?,
            val: input.parse()?,
        })
    }
}

#[derive(Clone)]
#[cfg_attr(feature = "testing", derive(Debug))]
pub struct ContainerAttributes(pub Punctuated<ContainerAttribute, Token![,]>);
impl Parse for ContainerAttributes {
    fn parse(input: ParseStream) -> Result<ContainerAttributes> {
        let punctuated = Punctuated::parse_terminated(input)?;
        Ok(ContainerAttributes(punctuated))
    }
}
impl ContainerAttributes {
    pub fn seek_val(&self, find: &str) -> Option<&syn::LitStr> {
        self.0.iter().find_map(|attr| {
            if attr.ident == find {
                Some(&attr.val)
            } else {
                None
            }
        })
    }
}

/// An attribute is a doc comment above a struct field identifier.
///
/// An attribute is either something we care about, or something we ignore.
///
/// If an attribute is something we care about, we store the key value pairs as
/// field configuration options. We also consume the attribute and do not
/// render the attribute.
///
/// If an attribute is something we do not care about, we ignore it and render
/// it back verbatim.
#[derive(Clone)]
#[cfg_attr(feature = "testing", derive(Debug))]
pub enum Attribute {
    Ours(AttributeOurs),
    Ignore(AttributeDoc),
}

impl Attribute {
    pub fn ours(&self) -> Option<&AttributeOurs> {
        match &self {
            Attribute::Ours(o) => Some(o),
            _ => None,
        }
    }
    pub fn ignore(&self) -> Option<&AttributeDoc> {
        match &self {
            Attribute::Ignore(i) => Some(i),
            _ => None,
        }
    }
    pub fn default(&self) -> Option<&DefaultLit> {
        self.ours()
            .filter(|meta| meta.meta.key == "default")
            .map(|meta| &meta.meta.val)
    }
    pub fn len(&self) -> Option<LitInt> {
        self.ours()
            .filter(|meta| meta.meta.key == "len")
            .map(|meta| &meta.meta.val)
            .and_then(|val| val.parse().ok())
    }
}

impl Parse for Attribute {
    fn parse(input: ParseStream) -> Result<Self> {
        let doc: AttributeDoc = input.parse()?;
        match doc.inner.parse::<MaybeOurs>()? {
            MaybeOurs::Ours(meta) => Ok(Attribute::Ours(AttributeOurs { doc, meta })),
            MaybeOurs::Ignore(_) => Ok(Attribute::Ignore(doc)),
        }
    }
}

impl ToTokens for Attribute {
    fn to_tokens(&self, toks: &mut TokenStream) {
        match self {
            Attribute::Ours(_) => unimplemented!(),
            Attribute::Ignore(i) => i.to_tokens(toks),
        }
    }
}

#[derive(Clone)]
#[cfg_attr(feature = "testing", derive(Debug))]
pub struct AttributeDoc {
    pub pound: Token![#],
    pub bracket: Bracket,
    pub doc: Ident,
    pub eq: Token![=],
    pub inner: syn::LitStr,
}

impl Parse for AttributeDoc {
    fn parse(input: ParseStream) -> Result<Self> {
        let inner;
        Ok(AttributeDoc {
            pound: input.parse()?,
            bracket: syn::bracketed!(inner in input),
            doc: inner.parse()?,
            eq: inner.parse()?,
            inner: inner.parse()?,
        })
    }
}

impl ToTokens for AttributeDoc {
    fn to_tokens(&self, toks: &mut TokenStream) {
        self.pound.to_tokens(toks);
        self.bracket.surround(toks, |toks| {
            self.doc.to_tokens(toks);
            self.eq.to_tokens(toks);
            self.inner.to_tokens(toks);
        })
    }
}

#[derive(Clone)]
#[cfg_attr(feature = "testing", derive(Debug))]
pub struct AttributeMeta {
    pub sbc: keyword::sbc,
    pub col: keyword::Col,
    pub key: Ident,
    pub eq: Token![=],
    pub val: DefaultLit,
}
impl Parse for AttributeMeta {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(AttributeMeta {
            sbc: input.parse()?,
            col: input.parse()?,
            key: input.parse()?,
            eq: input.parse()?,
            val: input.parse()?,
        })
    }
}

#[derive(Clone)]
#[cfg_attr(feature = "testing", derive(Debug))]
pub struct AttributeOurs {
    pub doc: AttributeDoc,
    pub meta: AttributeMeta, // <-- Content
}

#[cfg_attr(feature = "testing", derive(Debug))]
pub enum MaybeOurs {
    Ours(AttributeMeta),
    Ignore(TokenStream),
}
impl Parse for MaybeOurs {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(keyword::sbc) && input.peek2(keyword::Col) {
            Ok(MaybeOurs::Ours(input.parse()?))
        } else {
            Ok(MaybeOurs::Ignore(input.parse()?))
        }
    }
}

#[derive(Clone)]
#[cfg_attr(feature = "testing", derive(Debug))]
#[derive(Deserialize)] // TODO feature guard
#[serde(try_from = "String")] // TODO feature guard
pub struct DefaultLit(pub TokenStream);
impl DefaultLit {
    pub fn is_array(&self) -> bool {
        self.parse::<ExprArray>().map(|_| true).unwrap_or(false)
    }
    pub fn parse<T: Parse>(&self) -> Result<T> {
        syn::parse2(self.0.clone())
    }
}

impl std::convert::TryFrom<String> for DefaultLit {
    type Error = proc_macro2::LexError;
    fn try_from(s: String) -> std::result::Result<DefaultLit, Self::Error> {
        s.parse().map(DefaultLit)
    }
}

impl From<DefaultLit> for TokenStream {
    fn from(attr: DefaultLit) -> TokenStream {
        attr.0
    }
}

impl Parse for DefaultLit {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(DefaultLit(input.parse()?))
    }
}

impl ToTokens for DefaultLit {
    fn to_tokens(&self, toks: &mut TokenStream) {
        self.0.to_tokens(toks);
    }
}

#[derive(Clone)]
#[cfg_attr(feature = "testing", derive(Debug))]
pub struct Attributes(pub Vec<Attribute>);
impl Attributes {
    /// Helper to look through an array of attributes and find a "length" prop
    pub fn seek_len_lit(&self) -> Option<syn::LitInt> {
        self.0.iter().find_map(|attr| attr.len())
    }

    /// Helper to look through an array of attributes and find a "length" prop and convert to usize
    pub fn seek_len(&self) -> usize {
        self.seek_len_lit()
            .and_then(|lit| lit.base10_digits().parse().ok())
            .unwrap_or(0)
    }

    pub fn seek_default(&self) -> Option<&DefaultLit> {
        self.0.iter().find_map(|attr| attr.default())
    }
}

impl From<Vec<Attribute>> for Attributes {
    fn from(vec: Vec<Attribute>) -> Attributes {
        Attributes(vec)
    }
}

impl From<Attributes> for Vec<Attribute> {
    fn from(attr: Attributes) -> Vec<Attribute> {
        attr.0
    }
}
