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

pub use serde;
use serde_bindgen_core::binding;
use serde_json_core;

#[binding(prefix = "test")]
pub struct Remote<'a> {
    ///sbc: len = 4
    id0: &'a str,
}

#[binding(prefix = "test")]
pub struct Foo<'a> {
    id0: u8,
    id1: i8,
    id2: u16,
    id3: i16,
    id4: u32,
    id5: i32,
    id6: bool,
    /// sbc: len = 5
    id7: &'a str,
    id8: Remote<'a>,
    id9: Remote<'a>,
    id10: [u8; 3],
    id11: [Remote<'a>; 3],
    /// sbc: len = 3
    id12: [[&'a str; 3]; 2],
}

const DATA: &'static str = r#"
{
    "id0": 255,
    "id1": -128,
    "id2": 65535,
    "id3": -32767,
    "id4": 4294967295,
    "id5": -2147483647,
    "id6": false,
    "id7": "12345",
    "id8": {"id0": "1234"},
    "id9": {"id0": "5678"},
    "id10": [255,255,255],
    "id11": [{"id0": "1234"},{"id0": "1234"},{"id0": "1234"}],
    "id12": [["123","456","789"],["123","456","789"]]
}
"#;

#[test]
fn can_calculate_weight() {
    let mut foo = std::mem::MaybeUninit::<Foo>::uninit();
    let l = DATA.len();
    let p = DATA.as_ptr() as *const u8;
    let ret = unsafe { test_parse_foo(&mut *foo.as_mut_ptr(), p, l) };
    assert!(ret > 0);

    let foo = unsafe { foo.assume_init() };
    let mut buffer: [u8; 2096] = [0; 2096];
    let mut len = 2096;

    let ret = unsafe { test_print_foo_borrowed(&foo, &mut *buffer.as_mut_ptr(), &mut len) };
    assert_eq!(ret, 0);
    assert_eq!(len, FOO_MAX_LEN);
}

#[test]
fn can_calcualte_weight_when_renamed() {
    // TODO
}
