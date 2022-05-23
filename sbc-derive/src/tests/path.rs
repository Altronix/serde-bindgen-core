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

use crate::path::PathNamed;
use syn::parse_quote;

#[test]
fn can_parse_path_named_leading_colon() {
    let n: PathNamed = parse_quote!(::foo::bar);
    assert!(n.leading_colon.is_some());
    assert_eq!(n.ident, "bar");
    assert_eq!(n.segments.len(), 1);
    assert!(n.generics.is_none());
}

#[test]
fn can_parse_path_named_single_ident() {
    let n: PathNamed = parse_quote!(u16);
    assert!(n.leading_colon.is_none());
    assert_eq!(n.ident, "u16");
    assert_eq!(n.segments.len(), 0);
    assert!(n.generics.is_none());
}

#[test]
fn can_parse_path_named_with_generics() {
    let n: PathNamed = parse_quote!(hi<'a>);
    let expect: syn::Path = parse_quote!(hi<'a>);
    let expect = quote::quote! {#expect};
    let quoted = quote::quote! {#n};
    assert!(n.leading_colon.is_none());
    assert_eq!(n.ident, "hi");
    assert_eq!(n.segments.len(), 0);
    assert!(n.generics.is_some());
    assert_eq!(expect.to_string(), quoted.to_string());
}

#[test]
fn can_parse_path_named_segmented_with_generics() {
    let n: PathNamed = parse_quote!(foo::bar::hi<'a>);
    assert!(n.leading_colon.is_none());
    assert_eq!(n.ident, "hi");
    assert_eq!(n.segments.len(), 2);
    assert!(n.generics.is_some());
}

#[test]
fn can_to_tokens_path() {
    let sample: PathNamed = parse_quote!(::foo::bar::Baz<'a>);
    let expect = quote::quote! {::foo::bar::Baz<'a>};
    let quoted = quote::quote! {#sample};
    assert_eq!(expect.to_string(), quoted.to_string());
}
