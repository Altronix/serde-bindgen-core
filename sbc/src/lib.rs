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

pub use serde;
pub use serde_bindgen_core_derive::binding;
pub use serde_json_core;
pub use serde_json_core::heapless;

#[inline]
fn safe_copy<T: Copy + Default, const N: usize>(src: &[T]) -> [T; N] {
    let mut ret: [T; N] = [Default::default(); N];
    if src.len() < N {
        ret[..src.len()].copy_from_slice(&src[..src.len()]);
    } else {
        ret.copy_from_slice(&src[..N]);
    }
    ret
}

pub trait SafeCopy<Item, const N: usize> {
    fn safe_copy(&self) -> [Item; N];
}

macro_rules! impl_safe_copy_for_slice {
    ($t:ty) => {
        impl<const N: usize> SafeCopy<$t, N> for &[$t] {
            #[inline]
            fn safe_copy(&self) -> [$t; N] {
                safe_copy(self)
            }
        }
    };
}
impl_safe_copy_for_slice!(u8);
impl_safe_copy_for_slice!(i8);

impl<const N: usize> SafeCopy<u8, N> for &str {
    fn safe_copy(&self) -> [u8; N] {
        let mut ret = self.as_bytes().safe_copy();
        ret[N - 1] = 0;
        ret
    }
}

#[cfg(not(feature = "testing"))]
#[cfg(not(feature = "full"))]
#[lang = "eh_personality"]
fn eh_personality() {}

#[cfg(not(feature = "testing"))]
#[cfg(not(feature = "full"))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
