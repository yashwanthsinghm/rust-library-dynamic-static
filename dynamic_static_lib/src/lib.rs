#![allow(unused_variables)]
fn main() {
    extern crate staticlib1;

    #[no_mangle]
    pub fn run() {
        println!("2. Running dymanic library calling static libray ");
        let res = staticlib1::add(3, 5); //calling the add function : staticlib
        println!("res of addition 3,5 ={}", res);
    }
}
