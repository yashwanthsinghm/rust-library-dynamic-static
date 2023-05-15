
#[macro_use]
extern crate dlopen_derive;
use dlopen::wrapper::{Container, WrapperApi};
extern crate staticlib1;

#[derive(WrapperApi)]
struct PluginApi {
    run: extern fn(),

}

pub fn run() {
    println!("Starting App");

    let plugin_api_wrapper: Container<PluginApi> = unsafe { Container::load("libraries/libdynamiclib.dylib") }.unwrap();
    plugin_api_wrapper.run(); // call the dynamiclib run function
    let plugin_api_wrapper2: Container<PluginApi> = unsafe { Container::load("libraries/libdynamic_static_lib.dylib") }.unwrap();
    plugin_api_wrapper2.run();  // call the dynamic_static_lib run function
    //let mut res = plugin_api_wrapper.sub(8,5);
    //println!(" subtraction 8,5 ={}", res);
     //res = plugin_api_wrapper.sub(3,10);
    //println!(" subtraction 3,10 = -{}", res);
}


fn main() {
    println!("Running from main application");
    run();// calling the run function
    println!("3. Running from static library in application");
    let res = staticlib1::add(1,2);    //calling the add function : staticlib
    println!("res of addition 1,2 ={}", res);
}
