#![allow(unused_variables)]

use std::f32::consts::E;
fn main() {
    #[no_mangle]
    pub fn run() {
        println!("1. Running from  dymanic library");
    }
    #[no_mangle]
    pub fn sub(x: i32, y: i32) -> i32 {
        x - y
    }
}
