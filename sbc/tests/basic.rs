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

// use sbc::serde;
use serde;
use serde_bindgen_core::binding;
use serde_json_core;

mod songs {
    use serde;
    use serde_bindgen_core::binding;
    use serde_json_core;

    #[binding]
    pub struct RocketsShootBye<'a> {
        /// sbc: default = "rockets shoot bye"
        /// sbc: len = 33
        b0: &'a str,
        b1: u8,
        b2: [i32; 3],
        b4: [bool; 4],
    }
}

#[binding]
pub struct DeckTheHalls<'a> {
    /// sbc: default = "deck the halls"
    /// sbc: len = 33
    b0: &'a str,
    b1: u8,
    b2: [i32; 3],
    b4: [bool; 4],
}

#[binding]
pub struct JingleBells<'a> {
    /// sbc: default = "jingle bells"
    /// sbc: len = 33
    b0: &'a str,
    b1: u8,
    b2: [i32; 3],
    b4: [bool; 4],
    b5: DeckTheHalls<'a>,
    b6: [DeckTheHalls<'a>; 3],
    b7: songs::RocketsShootBye<'a>,
    b8: [songs::RocketsShootBye<'a>; 4],
}

fn main() {}
