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

use syn::parse::{Error, Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::Generics;
use syn::Ident;
use syn::ImplGenerics;
use syn::Token;
use syn::TypeGenerics;
use syn::WhereClause;

use proc_macro2::TokenStream;
use quote::ToTokens;

use heck::AsShoutySnakeCase;

use super::utils;

#[derive(Clone)]
#[cfg_attr(feature = "testing", derive(PartialEq, Debug))]
pub struct PathNamed {
    pub leading_colon: Option<Token![::]>,
    pub segments: Punctuated<Ident, Token![::]>,
    pub ident: Ident,
    pub generics: Option<Generics>,
}

impl PathNamed {
    pub fn is_primative(&self) -> bool {
        if self.ident == "i8"
            || self.ident == "i16"
            || self.ident == "i32"
            || self.ident == "u8"
            || self.ident == "u16"
            || self.ident == "u32"
            || self.ident == "bool"
        {
            true
        } else {
            false
        }
    }

    pub fn stackify(&mut self) -> &mut Self {
        self.rename(&format!("{}Owned", self.ident))
            .strip_generics();
        self
    }

    pub fn into_shouty_max_len(mut self) -> Self {
        let max_len = AsShoutySnakeCase(format!("{}_MAX_LEN", self.ident)).to_string();
        self.rename(&max_len).strip_generics();
        self
    }

    pub fn parsify(&mut self) -> &mut Self {
        self.rename(&format!("{}", self.ident));
        self
    }

    pub fn rename(&mut self, name: &str) -> &mut Self {
        self.ident = Ident::new(name, self.ident.span());
        self
    }

    pub fn strip_generics(&mut self) -> &mut Self {
        self.generics = None;
        self
    }

    pub fn split_self_for_impl(&self) -> (Self, Self) {
        let mut owned = self.clone();
        let mut parse = self.clone();
        owned.stackify();
        parse.parsify();
        (owned, parse)
    }

    pub fn split_generics_for_impl(
        &self,
    ) -> (
        Option<ImplGenerics>,
        Option<TypeGenerics>,
        Option<&WhereClause>,
    ) {
        if let Some(gen) = &self.generics {
            let (i, t, w) = gen.split_for_impl();
            (Some(i), Some(t), w)
        } else {
            (None, None, None)
        }
    }
}

impl Parse for PathNamed {
    fn parse(mut input: ParseStream) -> Result<Self> {
        let leading_colon: Option<Token![::]> = utils::maybe(Token![::], &mut input)?;
        let mut segments: Punctuated<Ident, Token![::]> =
            Punctuated::parse_separated_nonempty(input)?;
        let ident = segments
            .pop()
            .ok_or_else(|| Error::new(input.span(), "Must have an identity"))?
            .into_value();
        let generics = utils::maybe(Token![<], &mut input)?;
        Ok(PathNamed {
            leading_colon,
            segments,
            ident,
            generics,
        })
    }
}

impl ToTokens for PathNamed {
    fn to_tokens(&self, toks: &mut TokenStream) {
        if let Some(lc) = self.leading_colon {
            lc.to_tokens(toks);
        }
        self.segments.to_tokens(toks);
        self.ident.to_tokens(toks);
        if let Some(g) = &self.generics {
            g.to_tokens(toks);
        }
    }
}

impl From<Ident> for PathNamed {
    fn from(ident: Ident) -> PathNamed {
        syn::parse_quote!(#ident)
    }
}
