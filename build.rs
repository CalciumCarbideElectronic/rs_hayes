extern crate cc;
use std::env;
fn main() {
    let test_dir = format!("{}/test", env::var("CARGO_MANIFEST_DIR").unwrap());

    cc::Build::new()
        .file(format!("{}/sys_dummy.c", test_dir))
        .flag("-Wno-unused-parameter")
        .compile("sysdummy");

    println!("cargo:rustc-link-search={}", test_dir);
}
