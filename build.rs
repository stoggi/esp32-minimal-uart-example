fn main() {
    // Tell the linker to use esp-hal's linkall.x linker script
    println!("cargo:rustc-link-arg=-Tlinkall.x");
}
