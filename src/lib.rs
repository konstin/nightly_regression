#![feature(proc_macro)]

extern crate nightly_regression_derive;

use nightly_regression_derive::noop_derive;

pub struct Foo {
    x: u32,
}

#[noop_derive]
pub fn info_print(message: &str) {
    let _a = 1;
    Foo { x: 42 };
    let _b = 1;
    println!("{}", message);
    let _c = 1;
}
