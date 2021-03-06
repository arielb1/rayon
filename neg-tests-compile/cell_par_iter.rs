extern crate rayon;

// Check that we can't use the par-iter API to access contents of an
// `Rc`.

use rayon::prelude::*;
use std::cell::Cell;

fn main() {
    let c = Cell::new(42_i32);
    (0_i32..1024).into_par_iter()
             .map(|_| c.get()) //~ ERROR Sync` is not satisfied
             .min(); //~ ERROR Sync` is not satisfied
}
