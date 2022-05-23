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

#![no_builtins]
#![feature(lang_items)]
#![cfg_attr(not(test), no_std)]
#![allow(dead_code)]
#![allow(unused)]

// use data::serde;
use serde_bindgen_core::*;
use serde_bindgen_core::serde;
use serde_bindgen_core::serde_json_core;
use serde_bindgen_core_derive::binding;

#[binding]
pub struct Bar<'a> {
    /// sbc: default = "hi"
    /// sbc: len = 3
    id0: &'a str,
}

#[binding(prefix = "foo")]
pub struct Foo<'a> {
    /// sbc: default = 12
    id0: u8,
    id1: Bar<'a>,
    id2: [Bar<'a>; 3],
    /// sbc: default = 3
    id3: [u32; 3],
    /// sbc: default = "hello"
    /// sbc: len = 12
    id4: &'a str,
}

fn main() {}
