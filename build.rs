extern crate cmake;

use cmake::Config;

fn main() {
    let dst = Config::new("parser_lib").build();

    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=parser");
}