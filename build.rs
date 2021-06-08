use std::env;

fn main() {
    println!("cargo:rustc-flags=-l dylib=stdc++");

    let matrix_lib = env::var("MATRIX_LIB").unwrap_or("$HOME/Developer/rpi-rgb-matrix/lib".to_owned());

    println!("cargo:rustc-link-search=/{}", matrix_lib);
}
