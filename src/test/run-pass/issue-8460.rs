// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// ignore-emscripten no threads support
// ignore-pretty : (#23623) problems when  ending with // comments

#![feature(rustc_attrs, stmt_expr_attributes, zero_one)]

use std::num::Zero;
use std::thread;

macro_rules! check {
    ($($e:expr),*) => {
        $(assert!(thread::spawn({
            #[rustc_no_mir] // FIXME #29769 MIR overflow checking is TBD.
            move|| { $e; }
        }).join().is_err());)*
    }
}

#[rustc_no_mir] // FIXME #29769 MIR overflow checking is TBD.
fn main() {
    check![
        isize::min_value() / -1,
        i8::min_value() / -1,
        i16::min_value() / -1,
        i32::min_value() / -1,
        i64::min_value() / -1,
        1isize / isize::zero(),
        1i8 / i8::zero(),
        1i16 / i16::zero(),
        1i32 / i32::zero(),
        1i64 / i64::zero(),
        isize::min_value() % -1,
        i8::min_value() % -1,
        i16::min_value() % -1,
        i32::min_value() % -1,
        i64::min_value() % -1,
        1isize % isize::zero(),
        1i8 % i8::zero(),
        1i16 % i16::zero(),
        1i32 % i32::zero(),
        1i64 % i64::zero()
    ];
}
