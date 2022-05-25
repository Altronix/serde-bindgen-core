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

    #[derive(Debug, PartialEq, Clone)]
    #[binding]
    pub struct RocketsShootBye<'a> {
        /// sbc: default = "rockets"
        /// sbc: len = 33
        pub b0: &'a str,
        /// sbc: default = 30
        pub b1: u8,
        /// sbc: default = 3
        pub b2: [i32; 3],
        /// sbc: default = false
        pub b3: [bool; 4],
    }
}

#[derive(Debug, PartialEq, Clone)]
#[binding]
pub struct DeckTheHalls<'a> {
    /// sbc: default = "deck"
    /// sbc: len = 33
    pub b0: &'a str,
    /// sbc: default = 20
    pub b1: u8,
    /// sbc: default = 2
    pub b2: [i32; 3],
    pub b3: [bool; 4],
}

#[derive(Debug, PartialEq, Clone)]
#[binding(prefix = "test")]
pub struct JingleBells<'a> {
    /// sbc: default = "jingle"
    /// sbc: len = 33
    pub b0: &'a str,
    /// sbc: default = 10
    pub b1: u8,
    pub b2: [i32; 3],
    pub b3: [bool; 4],
    pub b4: DeckTheHalls<'a>,
    pub b5: [DeckTheHalls<'a>; 3],
    pub b6: songs::RocketsShootBye<'a>,
    pub b7: [songs::RocketsShootBye<'a>; 3],
    /// sbc: default = "jingle"
    /// sbc: len = 8
    pub b8: [&'a str; 3],
    /// sbc: default = "jingle"
    /// sbc: len = 8
    pub b9: [[&'a str; 3]; 2],
    /// sbc: default = [1, 2, 3]
    pub b10: [u8; 3],
    /// uninitialized test
    pub b11: &'a str,
}

const DATA: &'static str = r#"
{
  "b0": "b0",
  "b1": 0,
  "b2": [1, 2, 3],
  "b3": [true, false, false, false],
  "b4": {
      "b0": "b4.b0",
      "b1": 4,
      "b2": [5, 6, 7],
      "b3": [false, true, false, false]
  },
  "b5": [
    {
      "b0": "b5[0].b0",
      "b1": 8,
      "b2": [9, 10, 11],
      "b3": [false, false, true, false]
      },
    {
      "b0": "b5[1].b0",
      "b1": 12,
      "b2": [13, 14, 15],
      "b3": [false, false, false, true]
    },
    {
      "b0": "b5[2].b0",
      "b1": 16,
      "b2": [17, 18, 19],
      "b3": [true, false, false, false]
    }
  ],
  "b6": {
      "b0": "b6.b0",
      "b1": 20,
      "b2": [21, 22, 23],
      "b3": [false, true, false, false]
  },
  "b7": [
    {
      "b0": "b7[0].b0",
      "b1": 24,
      "b2": [25, 26, 27],
      "b3": [false, false, true, false]
    },
    {
      "b0": "b7[1].b0",
      "b1": 28,
      "b2": [29, 30, 31],
      "b3": [false, false, false, true]
    },
    {
      "b0": "b7[2].b0",
      "b1": 32,
      "b2": [33, 34, 35],
      "b3": [true, false, false, false]
    }
  ],
  "b8": ["apple", "banana", "car"],
  "b9": [
    ["apple", "banana", "car"],
    ["apple", "banana", "car"]
    ],
  "b10": [36, 37, 38],
  "b11": "apple"
}
"#;

const ROOT: JingleBells = JingleBells {
    b0: "b0",
    b1: 0,
    b2: [1, 2, 3],
    b3: [true, false, false, false],
    b4: DeckTheHalls {
        b0: "b4.b0",
        b1: 4,
        b2: [5, 6, 7],
        b3: [false, true, false, false],
    },
    b5: [
        DeckTheHalls {
            b0: "b5[0].b0",
            b1: 8,
            b2: [9, 10, 11],
            b3: [false, false, true, false],
        },
        DeckTheHalls {
            b0: "b5[1].b0",
            b1: 12,
            b2: [13, 14, 15],
            b3: [false, false, false, true],
        },
        DeckTheHalls {
            b0: "b5[2].b0",
            b1: 16,
            b2: [17, 18, 19],
            b3: [true, false, false, false],
        },
    ],
    b6: songs::RocketsShootBye {
        b0: "b6.b0",
        b1: 20,
        b2: [21, 22, 23],
        b3: [false, true, false, false],
    },
    b7: [
        songs::RocketsShootBye {
            b0: "b7[0].b0",
            b1: 24,
            b2: [25, 26, 27],
            b3: [false, false, true, false],
        },
        songs::RocketsShootBye {
            b0: "b7[1].b0",
            b1: 28,
            b2: [29, 30, 31],
            b3: [false, false, false, true],
        },
        songs::RocketsShootBye {
            b0: "b7[2].b0",
            b1: 32,
            b2: [33, 34, 35],
            b3: [true, false, false, false],
        },
    ],
    b8: ["apple", "banana", "car"],
    b9: [["apple", "banana", "car"], ["apple", "banana", "car"]],
    b10: [36, 37, 38],
    b11: "apple",
};

macro_rules! stringify {
    ($bytes:expr) => {
        std::str::from_utf8($bytes).unwrap().trim_end_matches('\0')
    };
}

fn assert_parsed(f: &JingleBells) {
    assert_eq!(f.b0, "b0");
    assert_eq!(f.b1, 0);
    assert_eq!(f.b2, [1, 2, 3]);
    assert_eq!(f.b3, [true, false, false, false]);
    assert_eq!(f.b4.b0, "b4.b0");
    assert_eq!(f.b4.b1, 4);
    assert_eq!(f.b4.b2, [5, 6, 7]);
    assert_eq!(f.b4.b3, [false, true, false, false]);
    assert_eq!(f.b5[0].b0, "b5[0].b0");
    assert_eq!(f.b5[0].b1, 8);
    assert_eq!(f.b5[0].b2, [9, 10, 11]);
    assert_eq!(f.b5[0].b3, [false, false, true, false]);
    assert_eq!(f.b5[1].b0, "b5[1].b0");
    assert_eq!(f.b5[1].b1, 12);
    assert_eq!(f.b5[1].b2, [13, 14, 15]);
    assert_eq!(f.b5[1].b3, [false, false, false, true]);
    assert_eq!(f.b5[2].b0, "b5[2].b0");
    assert_eq!(f.b5[2].b1, 16);
    assert_eq!(f.b5[2].b2, [17, 18, 19]);
    assert_eq!(f.b5[2].b3, [true, false, false, false]);
    assert_eq!(f.b6.b0, "b6.b0");
    assert_eq!(f.b6.b1, 20);
    assert_eq!(f.b6.b2, [21, 22, 23]);
    assert_eq!(f.b6.b3, [false, true, false, false]);
    assert_eq!(f.b7[0].b0, "b7[0].b0");
    assert_eq!(f.b7[0].b1, 24);
    assert_eq!(f.b7[0].b2, [25, 26, 27]);
    assert_eq!(f.b7[0].b3, [false, false, true, false]);
    assert_eq!(f.b7[1].b0, "b7[1].b0");
    assert_eq!(f.b7[1].b1, 28);
    assert_eq!(f.b7[1].b2, [29, 30, 31]);
    assert_eq!(f.b7[1].b3, [false, false, false, true]);
    assert_eq!(f.b7[2].b0, "b7[2].b0");
    assert_eq!(f.b7[2].b1, 32);
    assert_eq!(f.b7[2].b2, [33, 34, 35]);
    assert_eq!(f.b7[2].b3, [true, false, false, false]);
    assert_eq!(f.b8[0], "apple");
    assert_eq!(f.b8[1], "banana");
    assert_eq!(f.b8[2], "car");
    assert_eq!(f.b9[0][0], "apple");
    assert_eq!(f.b9[0][1], "banana");
    assert_eq!(f.b9[0][2], "car");
    assert_eq!(f.b9[1][0], "apple");
    assert_eq!(f.b9[1][1], "banana");
    assert_eq!(f.b9[1][2], "car");
    assert_eq!(f.b10, [36, 37, 38]);
    assert_eq!(f.b11, "apple");
}

#[test]
fn can_parse() {
    let mut f = std::mem::MaybeUninit::<JingleBells>::uninit();
    let l = DATA.len();
    let p = DATA.as_ptr() as *const u8;
    let ret = unsafe { test_parse_jingle_bells(&mut *f.as_mut_ptr(), p, l) };
    let f = unsafe { f.assume_init() };
    assert_eq!(ret, l as i32);
    assert_parsed(&f);
}

#[test]
fn can_print() {
    // Print test data
    let mut bytes: [u8; 2048] = [0; 2048];
    let mut l = 2048;
    let ret = unsafe { test_print_jingle_bells(&ROOT, bytes.as_mut_ptr(), &mut l) };

    // Parse test data
    let mut printed = std::mem::MaybeUninit::<JingleBells>::uninit();
    let ret = unsafe { test_parse_jingle_bells(&mut *printed.as_mut_ptr(), bytes.as_ptr(), l) };
    let printed = unsafe { printed.assume_init() };
    assert_eq!(ret, l as i32);
    assert_parsed(&printed);
}

#[test]
fn can_init() {
    let mut f = std::mem::MaybeUninit::<JingleBellsOwned>::uninit();
    unsafe { test_init_jingle_bells(&mut *f.as_mut_ptr()) };
    let f = unsafe { f.assume_init() };
    assert_eq!(stringify!(&f.b0), "jingle");
    assert_eq!(stringify!(&f.b4.b0), "deck");
    assert_eq!(stringify!(&f.b5[0].b0), "deck");
    assert_eq!(stringify!(&f.b5[1].b0), "deck");
    assert_eq!(stringify!(&f.b5[2].b0), "deck");
    assert_eq!(stringify!(&f.b6.b0), "rockets");
    assert_eq!(stringify!(&f.b7[0].b0), "rockets");
    assert_eq!(stringify!(&f.b7[1].b0), "rockets");
    assert_eq!(stringify!(&f.b7[2].b0), "rockets");
    assert_eq!(stringify!(&f.b8[0]), "jingle");
    assert_eq!(stringify!(&f.b8[1]), "jingle");
    assert_eq!(stringify!(&f.b8[2]), "jingle");
    assert_eq!(stringify!(&f.b9[0][0]), "jingle");
    assert_eq!(stringify!(&f.b9[0][1]), "jingle");
    assert_eq!(stringify!(&f.b9[0][2]), "jingle");
    assert_eq!(stringify!(&f.b9[1][0]), "jingle");
    assert_eq!(stringify!(&f.b9[1][1]), "jingle");
    assert_eq!(stringify!(&f.b9[1][2]), "jingle");

    assert_eq!(f.b1, 10);
    assert_eq!(f.b4.b1, 20);
    assert_eq!(f.b5[0].b1, 20);
    assert_eq!(f.b5[1].b1, 20);
    assert_eq!(f.b5[2].b1, 20);
    assert_eq!(f.b6.b1, 30);
    assert_eq!(f.b7[0].b1, 30);
    assert_eq!(f.b7[1].b1, 30);
    assert_eq!(f.b7[2].b1, 30);
    assert_eq!(f.b10, [1, 2, 3]);
    assert_eq!(f.b11, []);
}

#[test]
fn can_copy_into_owned() {
    let mut owned = std::mem::MaybeUninit::<JingleBellsOwned>::uninit();
    unsafe { test_copy_jingle_bells(&mut *owned.as_mut_ptr(), &ROOT) };
    let owned = unsafe { owned.assume_init() };
    assert_eq!(stringify!(&owned.b0), "b0");
    assert_eq!(owned.b1, 0);
    assert_eq!(owned.b2, [1, 2, 3]);
    assert_eq!(owned.b3, [true, false, false, false]);
    assert_eq!(stringify!(&owned.b4.b0), "b4.b0");
    assert_eq!(owned.b4.b1, 4);
    assert_eq!(owned.b4.b2, [5, 6, 7]);
    assert_eq!(owned.b4.b3, [false, true, false, false]);
    assert_eq!(stringify!(&owned.b5[0].b0), "b5[0].b0");
    assert_eq!(owned.b5[0].b1, 8);
    assert_eq!(owned.b5[0].b2, [9, 10, 11]);
    assert_eq!(owned.b5[0].b3, [false, false, true, false]);
    assert_eq!(stringify!(&owned.b5[1].b0), "b5[1].b0");
    assert_eq!(owned.b5[1].b1, 12);
    assert_eq!(owned.b5[1].b2, [13, 14, 15]);
    assert_eq!(owned.b5[1].b3, [false, false, false, true]);
    assert_eq!(stringify!(&owned.b5[2].b0), "b5[2].b0");
    assert_eq!(owned.b5[2].b1, 16);
    assert_eq!(owned.b5[2].b2, [17, 18, 19]);
    assert_eq!(owned.b5[2].b3, [true, false, false, false]);
    assert_eq!(stringify!(&owned.b6.b0), "b6.b0");
    assert_eq!(owned.b6.b1, 20);
    assert_eq!(owned.b6.b2, [21, 22, 23]);
    assert_eq!(owned.b6.b3, [false, true, false, false]);
    assert_eq!(stringify!(&owned.b7[0].b0), "b7[0].b0");
    assert_eq!(owned.b7[0].b1, 24);
    assert_eq!(owned.b7[0].b2, [25, 26, 27]);
    assert_eq!(owned.b7[0].b3, [false, false, true, false]);
    assert_eq!(stringify!(&owned.b7[1].b0), "b7[1].b0");
    assert_eq!(owned.b7[1].b1, 28);
    assert_eq!(owned.b7[1].b2, [29, 30, 31]);
    assert_eq!(owned.b7[1].b3, [false, false, false, true]);
    assert_eq!(stringify!(&owned.b7[2].b0), "b7[2].b0");
    assert_eq!(owned.b7[2].b1, 32);
    assert_eq!(owned.b7[2].b2, [33, 34, 35]);
    assert_eq!(owned.b7[2].b3, [true, false, false, false]);
    assert_eq!(stringify!(&owned.b8[0]), "apple");
    assert_eq!(stringify!(&owned.b8[1]), "banana");
    assert_eq!(stringify!(&owned.b8[2]), "car");
    assert_eq!(stringify!(&owned.b9[0][0]), "apple");
    assert_eq!(stringify!(&owned.b9[0][1]), "banana");
    assert_eq!(stringify!(&owned.b9[0][2]), "car");
    assert_eq!(stringify!(&owned.b9[1][0]), "apple");
    assert_eq!(stringify!(&owned.b9[1][1]), "banana");
    assert_eq!(stringify!(&owned.b9[1][2]), "car");
    assert_eq!(owned.b10, [36, 37, 38]);
    assert_eq!(owned.b11, []);
}

#[test]
fn can_reference_owned() {
  //let mut owned = std::mem::
}
