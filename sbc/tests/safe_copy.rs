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

use serde_bindgen_core::*;

#[test]
fn greater_than() {
    let var: &[u8] = &[0, 1, 2, 3, 4];
    let owned: [u8; 10] = var.safe_copy();
    assert_eq!(owned, [0, 1, 2, 3, 4, 0, 0, 0, 0, 0]);
}

#[test]
fn less_than() {
    let var: &[u8] = &[0, 1, 2, 3, 4];
    let owned: [u8; 3] = var.safe_copy();
    assert_eq!(owned, [0, 1, 2]);
}

#[test]
fn str_short() {
    let var: &str = "hi";
    let owned: [u8; 2] = var.safe_copy();
    assert_eq!(owned, [b'h', 0]);
    //assert_eq!(owned, [b'h', 0]);
}

#[test]
fn str_long() {
    let var: &str = "hi";
    let owned: [u8; 4] = var.safe_copy();
    assert_eq!(owned, [b'h', b'i', 0, 0]);
    //assert_eq!(owned, [b'h', 0]);
}

#[test]
fn str_eq() {
    let var: &str = "hi";
    let owned: [u8; 3] = var.safe_copy();
    assert_eq!(owned, [b'h', b'i', 0]);
    //assert_eq!(owned, [b'h', 0]);
}

// #[test]
// fn one() {
//     let var: u8 = 3;
//     let owned: u8 = var.safe_copy()[0];
// }
