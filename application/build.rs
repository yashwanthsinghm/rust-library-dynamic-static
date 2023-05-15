fn main() {
    // Add our staticlib build directory to the lib search path.
    println!("cargo:rustc-link-search=../staticlib1/target/debug");
    // Add the staticlib dependency directory to the lib search path.
    // This is may be required if the staticlib depends on any external "derive"
    println!("cargo:rustc-link-search=../staticlib1/target/debug/deps");
    // Link to the `staticlib` crate library. This tells cargo to actually link
    // to the `application` crate that we include using `extern crate staticlib;`.
    //println!("cargo:rustc-link-lib=staticlib1");
}