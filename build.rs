use std::env;

fn main() {
    println!("cargo:rustc-flags=-l dylib=stdc++");

    let matrix_lib = env::var("MATRIX_LIB").expect("Must specify a path for the RGB Matrix Library");

    println!("cargo:rustc-link-search=/{}", matrix_lib);
}
