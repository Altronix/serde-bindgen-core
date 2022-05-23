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

mod field;
mod path;
mod attribute;

use crate::context::{Context, ImplDefault, ImplFrom};
use crate::field::Field;
use std::matches;
use syn::parse_quote;

/*
macro_rules! lit_is {
    ($var:expr, $cmp:literal) => {
        $var.base10_digits().parse().unwrap_or(0) == $cmp
    };
}
*/

#[test]
fn can_parse_struct() {
    let s: Context = parse_quote!(
        pub struct Foo<'a> {
            pub id_0: i8,
            /// sbc: default = 3
            pub id_1: i16,
            pub id_2: i32,
            pub id_3: u8,
            pub id_4: u16,
            pub id_5: u32,
            pub id_6: bool,
            pub id_7: Foo<'a>,
            pub id_8: bar::Foo,
            pub id_9: ::Moo<'a>,
            /// garbage
            /// sbc: default = "hello world"
            /// more garbage
            /// sbc: len = 12
            /// ignore me
            pub id_10: &'a str,
            pub id_11: [bool; 6],
            pub id_12: [Foo; 5],
            pub id_13: [::bar::Foo<'a>; 8],
            // pub id_14: [&'a str; 3],
        }
    );
    // Validate struct
    assert_eq!(s.path.ident, "Foo");
    assert!(s.tok_vis.is_some());
    assert!(s.path.generics.is_some());

    // Validate fields (We use alias to make test "easier" to read)
    // use super::field::FieldType::Array as AR;
    use super::field::FieldType::Primative as PR;
    use super::field::FieldType::RefStr as RS;
    use super::field::FieldType::Struct as NS;
    // use super::field::FieldTypeArray::Primative as FAPR;
    // use super::field::FieldTypeArray::Struct as FANS;

    let f: Vec<Field> = s.fields.into_iter().collect();
    assert!(matches!(&f[0].ty, PR(ident) if ident=="i8"));
    assert!(matches!(&f[1].ty, PR(ident) if ident=="i16"));
    assert!(matches!(&f[2].ty, PR(ident) if ident=="i32"));
    assert!(matches!(&f[3].ty, PR(ident) if ident=="u8"));
    assert!(matches!(&f[4].ty, PR(ident) if ident=="u16"));
    assert!(matches!(&f[5].ty, PR(ident) if ident=="u32"));
    assert!(matches!(&f[6].ty, PR(ident) if ident=="bool"));
    assert!(matches!(&f[7].ty, NS(p) if p.ident=="Foo" && p.generics.is_some()));
    assert!(matches!(&f[8].ty, NS(p) if p.ident=="Foo" && p.generics.is_none()));
    assert!(matches!(&f[9].ty, NS(p) if p.ident=="Moo" && p.generics.is_some()));
    assert!(matches!(&f[10].ty, RS(..)));
    /*
    assert!(matches!(&f[11].ty, AR(FAPR(_,path,l)) if path.ident=="bool" && lit_is!(l,6)));
    assert!(
        matches!(&f[12].ty, AR(FANS(_,p,l)) if p.generics.is_none() && p.ident=="Foo" && lit_is!(l,5))
    );
    assert!(
        matches!(&f[13].ty, AR(FANS(_,p,l)) if p.generics.is_some() && p.ident=="Foo" && lit_is!(l,8))
    );
    */

    // TODO - read default and len on id_10

    assert_eq!(f[0].ident, "id_0");
    assert_eq!(f[1].ident, "id_1");
    assert_eq!(f[2].ident, "id_2");
    assert_eq!(f[3].ident, "id_3");
    assert_eq!(f[4].ident, "id_4");
    assert_eq!(f[5].ident, "id_5");
    assert_eq!(f[6].ident, "id_6");
    assert_eq!(f[7].ident, "id_7");
    assert_eq!(f[8].ident, "id_8");
    assert_eq!(f[9].ident, "id_9");
    assert_eq!(f[10].ident, "id_10");
    assert_eq!(f[11].ident, "id_11");
    assert_eq!(f[12].ident, "id_12");
    assert_eq!(f[13].ident, "id_13");
}

#[test]
fn can_to_tokens_struct() {
    let s: Context = parse_quote!(
        pub struct Foo<'a> {
            /// sbc: len = 22
            id_0: &'a str,
            id_1: Bar,
            id_2: ::Baz<'a>,
            id_3: [::Baz<'a>; 10],
        }
    );
    let expect = quote::quote! {
        pub struct Foo <'a> {
            id_0: &'a str,
            id_1: Bar,
            id_2: ::Baz<'a>,
            id_3: [::Baz<'a>; 10],
        }
    };
    let quoted = quote::quote! {#s};
    assert_eq!(expect.to_string(), quoted.to_string());
}

#[test]
fn can_stackify() {
    let mut original: Context = parse_quote!(
        pub struct Foo<'a> {
            /// sbc: len = 22
            id_0: &'a str,
            id_1: Bar,
            id_2: ::Baz<'a>,
            id_3: [::Baz<'a>; 10],
            id_4: i32,
            id_5: u16,
        }
    );
    let expect = quote::quote! {
        pub struct FooOwned {
            id_0: [u8; 22],
            id_1: BarOwned,
            id_2: ::BazOwned,
            id_3: [::BazOwned; 10],
            id_4: i32,
            id_5: u16,
        }
    };
    original.stackify();
    let quoted = quote::quote! {#original};
    assert_eq!(expect.to_string(), quoted.to_string());
}

#[test]
fn can_impl_default() {
    let original: Context = parse_quote!(
        pub struct Foo<'a> {
            /// sbc: default = "hello"
            /// sbc: len = 22
            id_0: &'a str,
            id_1: Bar,
            id_2: ::Baz<'a>,
            id_3: [::Baz<'a>; 2],
            /// sbc: default = 3
            id_4: i32,
            /// sbc: default = 6
            id_5: u16,
        }
    );
    let expect = quote::quote!(
        impl Default for FooOwned {
            fn default() -> FooOwned {
                FooOwned {
                    id_0: serde_bindgen_core::SafeCopy::safe_copy(&"hello"),
                    id_1: Default::default(),
                    id_2: Default::default(),
                    id_3: [Default::default(),Default::default()],
                    id_4: 3,
                    id_5: 6
                }
            }
        }
    );
    let impl_default = ImplDefault::new(&original.path, &original.fields);
    let quoted = quote::quote! {#impl_default};
    assert_eq!(expect.to_string(), quoted.to_string());
}

#[test]
fn can_impl_from() {
    let original: Context = parse_quote!(
        pub struct Foo<'a> {
            id0: &'a str,
            id1: [u8; 2],
            id2: Baz<'a>,
            id3: [Baz<'a>; 2],
        }
    );
    #[rustfmt::skip]
    let expect: syn::ItemImpl = parse_quote!(
        impl<'a> From<&Foo<'a>> for FooOwned {
            fn from(s: &Foo<'a>) -> FooOwned {
                FooOwned {
                    id0: serde_bindgen_core::SafeCopy::safe_copy(&s.id0),
                    id1: [s.id1[0],s.id1[1]],
                    id2: From::from(&s.id2),
                    id3: [From::from(&s.id3[0]), From::from(&s.id3[1])]
                }
            }
        }
    );
    let impl_from = ImplFrom::new(&original.path, &original.fields);
    let expect = quote::quote! {#expect};
    let quoted = quote::quote! {#impl_from};
    assert_eq!(expect.to_string(), quoted.to_string());
}

#[test]
fn can_binding_default() {
    let original: Context = parse_quote!(
        pub struct Foo<'a> {
            item: &'a str,
        }
    );
    let binding = original.binding_init("foo");
    let expect = quote::quote! {
        #[no_mangle]
        pub extern "C" fn foo_init_foo<'a>(dst: &mut FooOwned) {
            *dst=Default::default();
        }
    };
    let quoted = quote::quote! {#binding};
    assert_eq!(expect.to_string(), quoted.to_string());
}

#[test]
fn can_binding_copy() {
    let original: Context = parse_quote!(
        pub struct Foo<'a> {
            item: &'a str,
        }
    );
    let binding = original.binding_copy("foo");
    let expect = quote::quote! {
        #[no_mangle]
        pub extern "C" fn foo_copy_foo<'a>(dst: &mut FooOwned, src: &Foo<'a>) {
            *dst = From::from(src);
        }
    };
    let quoted = quote::quote! {#binding};
    assert_eq!(expect.to_string(), quoted.to_string());
}

#[test]
fn can_binding_parse() {
    let original: Context = parse_quote!(
        pub struct Foo<'a> {
            item: &'a str,
        }
    );
    let binding = original.binding_parse("foo");
    let expect = quote::quote! {
        #[no_mangle]
        pub extern "C" fn foo_parse_foo<'a>(dst: &mut Foo<'a>, bytes: *const u8, len: usize) -> i32 {
            let slice = unsafe { core::slice::from_raw_parts(bytes,len) };
            match serde_json_core::from_slice(&slice) {
                Ok((item,len)) => {
                    *dst = item;
                    len as i32
                },
                Err(_) => -1
            }
        }
    };
    let quoted = quote::quote! {#binding};
    assert_eq!(expect.to_string(), quoted.to_string());
}

#[test]
fn can_binding_print() {
    let original: Context = parse_quote!(
        pub struct Foo<'a> {
            item: &'a str,
        }
    );
    let binding = original.binding_print("foo");
    let expect = quote::quote! {
        #[no_mangle]
        pub extern "C" fn foo_print_foo<'a>(data: &Foo<'a>, bytes: *mut u8, len: &mut usize) -> i32 {
            let mut slice = unsafe { core::slice::from_raw_parts_mut(bytes,*len) };
            match serde_json_core::to_slice(data, &mut slice) {
                Ok(l) => {
                    *len = l;
                    0
                },
                Err(_) => -1
            }
        }
    };
    let quoted = quote::quote! {#binding};
    assert_eq!(expect.to_string(), quoted.to_string());
}
