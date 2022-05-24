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

use crate::field::{Field, FieldTypeArray, FieldTypeRef};
use syn::parse_quote;

#[test]
fn can_parse_field_opt_bool_true() {
    let attr: Field = parse_quote!(
        /// sbc: default = true
        id: bool
    );
    let lit: syn::LitBool = attr.attributes.0[0].default().unwrap().parse().unwrap();
    assert_eq!(lit.value(), true);
}

#[test]
fn can_parse_field_opt_bool_false() {
    let attr: Field = parse_quote!(
        /// sbc: default = false
        id: bool
    );
    let lit: syn::LitBool = attr.attributes.0[0].default().unwrap().parse().unwrap();
    assert_eq!(lit.value(), false);
}

#[test]
fn can_parse_field_opt_str() {
    let attr: Field = parse_quote!(
        /// sbc: default = "hello"
        /// sbc: len = 3
        id: &'a str
    );
    let init: syn::LitStr = attr.attributes.0[0].default().unwrap().parse().unwrap();
    let len = attr.attributes.0[1].len().unwrap();
    assert_eq!(init.value(), "hello");
    assert_eq!(len.base10_digits().parse::<i32>().unwrap(), 3);
}

#[test]
fn can_parse_field_opt_int() {
    let attr: Field = parse_quote!(
        /// sbc: default = 42
        id: u8
    );
    let init: syn::LitInt = attr.attributes.0[0].default().unwrap().parse().unwrap();
    assert_eq!(init.base10_digits(), "42");
}

#[test]
fn can_to_tokens_field_type_array_prim() {
    let sample: FieldTypeArray = parse_quote!([bool; 11]);
    let expect = quote::quote! {[bool;11]};
    let quoted = quote::quote! {#sample};
    assert_eq!(expect.to_string(), quoted.to_string());
}

#[test]
fn can_to_tokens_field_type_array_struct() {
    let sample: FieldTypeArray = parse_quote!([::foo::bar::Baz<'a>; 11]);
    let expect = quote::quote! {[::foo::bar::Baz<'a>;11]};
    let quoted = quote::quote! {#sample};
    assert_eq!(expect.to_string(), quoted.to_string());
}

#[test]
fn can_to_tokens_field_type_ref() {
    let sample: FieldTypeRef = parse_quote!(&'a str);
    let expect = quote::quote! {&'a str};
    let quoted = quote::quote! {#sample};
    assert_eq!(expect.to_string(), quoted.to_string());
}

#[test]
fn can_to_tokens_for_field_ref_str() {
    let sample: Field = parse_quote!(pub foo: &'a str);
    let expect = quote::quote! {pub foo: &'a str};
    let quoted = quote::quote! {#sample};
    assert_eq!(expect.to_string(), quoted.to_string());
}

#[test]
fn can_to_tokens_for_field_nested_struct() {
    let sample: Field = parse_quote!(pub foo: Jim);
    let expect = quote::quote! {pub foo: Jim};
    let quoted = quote::quote! {#sample};
    assert_eq!(expect.to_string(), quoted.to_string());
}

#[test]
fn can_to_tokens_field_prim() {
    let sample: Field = parse_quote!(pub foo: bool);
    let expect = quote::quote! {pub foo: bool};
    let quoted = quote::quote! {#sample};
    assert_eq!(expect.to_string(), quoted.to_string());
}

#[test]
fn can_to_tokens_for_field_array() {
    let sample: Field = parse_quote!(pub foo: [::baz<'a>; 11]);
    let expect = quote::quote! {pub foo: [::baz<'a>; 11]};
    let quoted = quote::quote! {#sample};
    assert_eq!(expect.to_string(), quoted.to_string());
}

#[test]
fn can_to_tokens_for_field_array_with_ignors() {
    let sample: Field = parse_quote!(
        /// sbc: len = 12
        /// garbage
        /// sbc: default = true
        /// more garbage
        pub foo: [::baz<'a>; 11]
    );
    let expect = quote::quote! {
        /// garbage
        /// more garbage
        pub foo: [::baz<'a>; 11]
    };
    let quoted = quote::quote! {#sample};
    assert_eq!(expect.to_string(), quoted.to_string());
}

#[test]
fn can_assignment_tokens_ref_str() {
    let sample: Field = parse_quote!(
      /// sbc: default = "initial foo"
      pub foo: &'a str
    );
    let expect = quote::quote!(foo: serde_bindgen_core::SafeCopy::safe_copy(&"initial foo"));
    let quoted = sample.assignment_tokens();
    assert_eq!(expect.to_string(), quoted.to_string());
}

#[test]
fn can_assignment_tokens_nested_struct() {
    let sample: Field = parse_quote!(pub foo: Baz);
    let expect = quote::quote!(foo: Default::default());
    let quoted = sample.assignment_tokens();
    assert_eq!(expect.to_string(), quoted.to_string());
}

#[test]
fn can_assignment_tokens_bool() {
    let sample: Field = parse_quote!(
       /// sbc: default = true
       pub foo: bool
    );
    let expect = quote::quote!(foo: true);
    let quoted = sample.assignment_tokens();
    assert_eq!(expect.to_string(), quoted.to_string());
}

#[test]
fn can_assignment_tokens_int() {
    let sample: Field = parse_quote!(
      /// sbc: default = 42
      pub foo: u8
    );
    let expect = quote::quote!(foo: 42);
    let quoted = sample.assignment_tokens();
    assert_eq!(expect.to_string(), quoted.to_string());
}

#[test]
fn can_assignment_tokens_array_struct() {
    let sample: Field = parse_quote!(foo: [Baz<'a>; 2]);
    let expect = quote::quote!(foo: [Default::default(),Default::default()]);
    let quoted = sample.assignment_tokens();
    assert_eq!(expect.to_string(), quoted.to_string());
}

#[test]
fn can_assignment_tokens_array_prim() {
    let sample: Field = parse_quote!(
        /// sbc: default = 42
        foo: [u32; 3]
    );
    let expect = quote::quote!(foo: [42,42,42]);
    let quoted = sample.assignment_tokens();
    assert_eq!(expect.to_string(), quoted.to_string());
}

#[test]
fn can_assignment_tokens_array_prim_recursive() {
    let sample: Field = parse_quote!(
        /// sbc: default = 42
        foo: [[u32; 2]; 3]
    );
    let expect = quote::quote!(foo: [[42,42],[42,42],[42,42]]);
    let quoted = sample.assignment_tokens();
    assert_eq!(expect.to_string(), quoted.to_string());
}

#[test]
fn can_from_tokens_ref_str() {
    let sample: Field = parse_quote!(
      /// sbc: default = "initial foo"
      pub foo: &'a str
    );
    let var = quote::format_ident!("s");
    let expect = quote::quote!(foo: serde_bindgen_core::SafeCopy::safe_copy(&s.foo));
    let quoted = sample.from_tokens(&var);
    assert_eq!(expect.to_string(), quoted.to_string());
}

#[test]
fn can_from_tokens_nested_struct() {
    let sample: Field = parse_quote!(pub foo: Baz<'a>);
    let var = quote::format_ident!("s");
    let expect = quote::quote!(foo: From::from(&s.foo));
    let quoted = sample.from_tokens(&var);
    assert_eq!(expect.to_string(), quoted.to_string());
}

#[test]
fn can_from_tokens_bool() {
    let sample: Field = parse_quote!(
       /// sbc: default = true
       pub foo: bool
    );
    let var = quote::format_ident!("s");
    let expect = quote::quote!(foo: s.foo);
    let quoted = sample.from_tokens(&var);
    assert_eq!(expect.to_string(), quoted.to_string());
}

#[test]
fn can_from_tokens_int() {
    let sample: Field = parse_quote!(
      /// sbc: default = 42
      pub foo: u8
    );
    let var = quote::format_ident!("s");
    let expect = quote::quote!(foo: s.foo);
    let quoted = sample.from_tokens(&var);
    assert_eq!(expect.to_string(), quoted.to_string());
}

#[test]
fn can_from_tokens_array_struct() {
    let sample: Field = parse_quote!(foo: [Baz<'a>; 2]);
    let var = quote::format_ident!("s");
    let expect = quote::quote!(foo: [From::from(&s.foo[0]),From::from(&s.foo[1])]);
    let quoted = sample.from_tokens(&var);
    assert_eq!(expect.to_string(), quoted.to_string());
}

#[test]
fn can_from_tokens_array_prim() {
    let sample: Field = parse_quote!(foo: [u8; 2]);
    let var = quote::format_ident!("s");
    let expect = quote::quote!(foo: [s.foo[0], s.foo[1]]);
    let quoted = sample.from_tokens(&var);
    assert_eq!(expect.to_string(), quoted.to_string());
}
