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

// quote
use quote::quote;
use quote::ToTokens;

// syn::
use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::Token;

// heck::
use heck::AsSnakeCase;

// proc_macro2
use proc_macro2::TokenStream;

use crate::field::Field;
use crate::path::PathNamed;
use crate::utils;

#[derive(Clone)]
pub(crate) struct Context {
    pub path: PathNamed,
    pub tok_vis: Option<Token![pub]>,
    pub tok_struct: Token![struct],
    pub tok_brace: syn::token::Brace,
    pub fields: Punctuated<Field, Token![,]>,
}

impl Context {
    pub fn weight<'a>(&'a self) -> (usize, Vec<(&'a PathNamed, usize)>) {
        // start with size of 2 for {} brackets
        let (mut len, remotes) = self.fields.iter().map(|field| field.weight()).fold(
            (2, Vec::new()),
            |(acc_len, mut acc_vec), (len, remote)| {
                if let Some(remote) = remote {
                    acc_vec.push(remote)
                }
                (acc_len + len, acc_vec)
            },
        );
        len += self.fields.len() - 1; // add a comma per field except the last
        (len, remotes)
    }

    pub fn stackify(&mut self) {
        self.path.stackify();
        self.fields.iter_mut().for_each(|f| f.stackify());
    }

    pub fn into_owned(mut self) -> Context {
        self.stackify();
        self
    }

    pub fn impl_from_owned(&self) -> ImplFromOwned {
        ImplFromOwned::new(&self.path, &self.fields)
    }

    pub fn impl_default(&self) -> ImplDefault {
        ImplDefault::new(&self.path, &self.fields)
    }

    pub fn impl_weight(&self) -> ImplWeight {
        let (weight, remotes) = self.weight();
        ImplWeight::new(&self.path, weight, remotes)
    }

    pub fn binding_copy<'a>(&'a self, prefix: &'a str) -> BindingCopy<'a> {
        BindingCopy::new(prefix, &self.path)
    }

    pub fn binding_init<'a>(&'a self, prefix: &'a str) -> BindingDefault<'a> {
        BindingDefault::new(prefix, &self.path)
    }

    pub fn binding_parse<'a>(&'a self, prefix: &'a str) -> BindingParse<'a> {
        BindingParse::new(prefix, &self.path)
    }

    pub fn binding_print<'a>(&'a self, prefix: &'a str) -> BindingPrint<'a> {
        BindingPrint::new(prefix, &self.path)
    }
}

impl Parse for Context {
    fn parse(mut input: ParseStream) -> Result<Self> {
        syn::Attribute::parse_inner(input)?;
        let inner;
        Ok(Context {
            tok_vis: utils::maybe(Token![pub], &mut input)?,
            tok_struct: input.parse()?,
            path: input.parse()?,
            tok_brace: syn::braced!(inner in input),
            fields: inner.parse_terminated(Field::parse)?,
        })
    }
}

impl ToTokens for Context {
    fn to_tokens(&self, toks: &mut TokenStream) {
        if let Some(vis) = self.tok_vis {
            vis.to_tokens(toks);
        }
        self.tok_struct.to_tokens(toks);
        self.path.to_tokens(toks);
        self.tok_brace.surround(toks, |toks| {
            self.fields.to_tokens(toks);
        })
    }
}

pub struct ImplDefault<'a> {
    pub path: &'a PathNamed,
    pub fields: &'a Punctuated<Field, Token![,]>,
}

impl<'a> ImplDefault<'a> {
    pub fn new(path: &'a PathNamed, fields: &'a Punctuated<Field, Token![,]>) -> ImplDefault<'a> {
        ImplDefault { path, fields }
    }
}

impl<'a> ToTokens for ImplDefault<'a> {
    fn to_tokens(&self, toks: &mut TokenStream) {
        let (owned, _ident) = self.path.split_self_for_impl();
        let assignment_tokens = self.fields.iter().map(|field| field.assignment_tokens());
        quote! {
            impl Default for #owned {
                fn default() -> #owned {
                    #owned {
                        #(#assignment_tokens),*
                    }
                }
            }
        }
        .to_tokens(toks);
    }
}

pub struct ImplWeight<'a> {
    pub path: &'a PathNamed,
    pub weight: usize,
    pub remotes: Vec<(&'a PathNamed, usize)>,
}

impl<'a> ImplWeight<'a> {
    pub fn new(
        path: &'a PathNamed,
        weight: usize,
        remotes: Vec<(&'a PathNamed, usize)>,
    ) -> ImplWeight<'a> {
        ImplWeight {
            path,
            weight,
            remotes,
        }
    }
}

impl<'a> ToTokens for ImplWeight<'a> {
    fn to_tokens(&self, toks: &mut TokenStream) {
        let (_other, mut ident) = self.path.split_self_for_impl();
        let weight = self.weight;
        let remotes = self
            .remotes
            .iter()
            .map(|(remote, n)| ((*remote).clone().into_shouty_max_len(), n))
            .fold(None, |acc, (remote, n)| Some(quote! {#acc + #remote * #n}));
        ident = ident.clone().into_shouty_max_len();
        quote! {
            pub const #ident: usize = #weight #remotes;
        }
        .to_tokens(toks);
    }
}

pub struct ImplFromOwned<'a> {
    pub path: &'a PathNamed,
    pub fields: &'a Punctuated<Field, Token![,]>,
}

impl<'a> ImplFromOwned<'a> {
    pub fn new(path: &'a PathNamed, fields: &'a Punctuated<Field, Token![,]>) -> ImplFromOwned<'a> {
        ImplFromOwned { path, fields }
    }
}

impl<'a> ToTokens for ImplFromOwned<'a> {
    fn to_tokens(&self, toks: &mut TokenStream) {
        let (other, ident) = self.path.split_self_for_impl();
        let (impl_generics, _ty_generics, where_clause) = self.path.split_generics_for_impl();
        let var = quote::format_ident!("s");
        let from_tokens = self
            .fields
            .iter()
            .map(|field| field.from_owned_tokens(&var));
        quote! {
            impl #impl_generics From<&#ident> for #other #where_clause {
                fn from(s: &#ident) -> #other {
                    #other {
                        #(#from_tokens),*
                    }
                }
            }
        }
        .to_tokens(toks);
    }
}

pub struct BindingDefault<'a> {
    ident: &'a PathNamed,
    prefix: &'a str,
}

impl<'a> BindingDefault<'a> {
    fn new(prefix: &'a str, ident: &'a PathNamed) -> BindingDefault<'a> {
        BindingDefault { ident, prefix }
    }
}

impl<'a> ToTokens for BindingDefault<'a> {
    fn to_tokens(&self, toks: &mut TokenStream) {
        let (owned, ident) = self.ident.split_self_for_impl();
        let (impl_generics, _, _) = self.ident.split_generics_for_impl();
        let name_fn = quote::format_ident!(
            "{}_init_{}",
            self.prefix,
            format!("{}", AsSnakeCase(format!("{}", ident.ident)))
        );
        quote! {
            #[no_mangle]
            pub extern "C" fn #name_fn #impl_generics(dst: &mut #owned)  {
                *dst = Default::default();
            }
        }
        .to_tokens(toks);
    }
}

pub struct BindingCopy<'a> {
    ident: &'a PathNamed,
    prefix: &'a str,
}

impl<'a> BindingCopy<'a> {
    fn new(prefix: &'a str, ident: &'a PathNamed) -> BindingCopy<'a> {
        BindingCopy { ident, prefix }
    }
}

impl<'a> ToTokens for BindingCopy<'a> {
    fn to_tokens(&self, toks: &mut TokenStream) {
        let (owned, ident) = self.ident.split_self_for_impl();
        let (impl_generics, _, _) = self.ident.split_generics_for_impl();
        let name_fn = quote::format_ident!(
            "{}_copy_{}",
            self.prefix,
            format!("{}", AsSnakeCase(format!("{}", ident.ident)))
        );
        quote! {
            #[no_mangle]
            pub extern "C" fn #name_fn #impl_generics(dst: &mut #owned, src: &#ident)  {
                *dst = From::from(src);
            }
        }
        .to_tokens(toks);
    }
}

pub struct BindingParse<'a> {
    ident: &'a PathNamed,
    prefix: &'a str,
}

impl<'a> BindingParse<'a> {
    fn new(prefix: &'a str, ident: &'a PathNamed) -> BindingParse<'a> {
        BindingParse { ident, prefix }
    }
}

impl<'a> ToTokens for BindingParse<'a> {
    fn to_tokens(&self, toks: &mut TokenStream) {
        let (_owned, ident) = self.ident.split_self_for_impl();
        let (impl_generics, _, _) = self.ident.split_generics_for_impl();
        let name_fn = quote::format_ident!(
            "{}_parse_{}",
            self.prefix,
            format!("{}", AsSnakeCase(format!("{}", ident.ident)))
        );
        quote! {
            #[no_mangle]
            pub extern "C" fn #name_fn #impl_generics(dst: &mut #ident, bytes: *const u8, len: usize) -> i32 {
                let slice = unsafe { core::slice::from_raw_parts(bytes, len) };
                match serde_json_core::from_slice(&slice) {
                    Ok((item, len))=> {
                        *dst = item;
                        len as i32
                    },
                    Err(_) => -1
                }
            }
        }
        .to_tokens(toks);
    }
}

pub struct BindingPrint<'a> {
    ident: &'a PathNamed,
    prefix: &'a str,
}

impl<'a> BindingPrint<'a> {
    fn new(prefix: &'a str, ident: &'a PathNamed) -> BindingPrint<'a> {
        BindingPrint { ident, prefix }
    }
}

impl<'a> ToTokens for BindingPrint<'a> {
    fn to_tokens(&self, toks: &mut TokenStream) {
        let (_owned, ident) = self.ident.split_self_for_impl();
        let (impl_generics, _, _) = self.ident.split_generics_for_impl();
        let name_fn = quote::format_ident!(
            "{}_print_{}",
            self.prefix,
            format!("{}", AsSnakeCase(format!("{}", ident.ident)))
        );
        quote! {
            #[no_mangle]
            pub extern "C" fn #name_fn #impl_generics(data: &#ident, bytes: *mut u8, len: &mut usize) -> i32 {
                let mut slice = unsafe { core::slice::from_raw_parts_mut(bytes, *len) };
                match serde_json_core::to_slice(data, &mut slice) {
                    Ok(l)=> {
                        *len = l;
                        0
                    },
                    Err(_) => -1
                }
            }
        }
        .to_tokens(toks);
    }
}
