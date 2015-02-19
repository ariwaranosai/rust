// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::marker;

struct invariant<'a> {
    marker: marker::InvariantLifetime<'a>
}

fn to_same_lifetime<'r>(b_isize: invariant<'r>) {
    let bj: invariant<'r> = b_isize;
}

fn to_longer_lifetime<'r>(b_isize: invariant<'r>) -> invariant<'static> {
    b_isize //~ ERROR mismatched types
}

fn main() {
}
