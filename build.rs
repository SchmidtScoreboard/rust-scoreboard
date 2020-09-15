fn main() {
    println!("cargo:rustc-flags=-l dylib=stdc++");

    println!("cargo:rustc-link-search=/home/pi/rpi-rgb-led-matrix/lib");
}
