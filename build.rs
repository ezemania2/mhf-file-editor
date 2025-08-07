fn main() {
    println!("cargo:rerun-if-changed=app.rc");
    println!("cargo:rustc-link-arg-bins=app.res");
} 