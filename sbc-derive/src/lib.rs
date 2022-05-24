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

#[cfg(test)]
mod tests;

mod attributes;
mod context;
mod field;
mod keyword;
mod path;
mod utils;

use attributes::AttributeConfig;
use context::Context;
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro_attribute]
pub fn binding(attr: TokenStream, item: TokenStream) -> TokenStream {
    let prefix = parse_macro_input!(attr as AttributeConfig)
        .0
        .map(|(_, _, s)| s.value())
        .unwrap_or("sbc".to_string());

    // Prse the callers decorated struct
    let ctx: Context = parse_macro_input!(item);

    // create an "owned" version of the struct. (no references)
    let owned = ctx.clone().into_owned();

    // create a const FOO: usize = max_len block
    let impl_weight = ctx.impl_weight();

    // create impl Default block
    let impl_default = ctx.impl_default();

    // create impl From block
    let impl_from = ctx.impl_from();

    // create binding for copy function
    let binding_copy = ctx.binding_copy(&prefix);

    // create binding for init function
    let binding_init = ctx.binding_init(&prefix);

    // create binding for parse function
    let binding_parse = ctx.binding_parse(&prefix);

    // create binding for parse function
    let binding_print = ctx.binding_print(&prefix);

    // render all the new items
    let quoted = quote! {
        #[no_mangle]
        #impl_weight
        #[repr(C)]
        #[derive(serde::Deserialize)]
        #[derive(serde::Serialize)]
        #[serde(crate="self::serde")]
        #ctx
        #[repr(C)]
        #owned
        #impl_default
        #impl_from
        #binding_copy
        #binding_init
        #binding_parse
        #binding_print
    };
    proc_macro::TokenStream::from(quoted)
}
