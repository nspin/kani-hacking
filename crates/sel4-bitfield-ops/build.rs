fn main() {
    println!("cargo::rustc-check-cfg=cfg(kani)");
    println!("cargo::rerun-if-changed=build.rs");
}
