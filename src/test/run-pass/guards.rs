// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

fn main() {
    let a =
        match 10 { x if x < 7 => { 1 } x if x < 11 => { 2 } 10 => { 3 } _ => { 4 } };
    assert (a == 2);

    let b =
        match {x: 10, y: 20} {
          x if x.x < 5 && x.y < 5 => { 1 }
          {x: x, y: y} if x == 10 && y == 20 => { 2 }
          {x: x, y: y} => { 3 }
        };
    assert (b == 2);
}
