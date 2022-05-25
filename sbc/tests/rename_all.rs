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

use serde;
use serde_bindgen_core::binding;
use serde_json_core;

#[binding(prefix = "test", rename_all = "camelCase")]
pub struct Foo {
    this_is_a_thing: u8,
}

#[test]
fn can_rename_all() {
    let mut parsed = std::mem::MaybeUninit::<FooBorrowed>::uninit();
    let data = "{\"thisIsAThing\":3}";
    let l = data.len();
    let p = data.as_ptr() as *const u8;
    let ret = unsafe { test_parse_foo(&mut *parsed.as_mut_ptr(), p, l) };
    assert_eq!(ret, l as i32);
    let parsed = unsafe { parsed.assume_init() };
    assert_eq!(parsed.this_is_a_thing, 3);
}
