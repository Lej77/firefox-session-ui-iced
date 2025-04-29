
fn main() {
    // https://doc.rust-lang.org/cargo/reference/build-scripts.html#change-detection
    println!("cargo::rerun-if-changed=build.rs"); // <- enable fine grained change detection.

    // https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-build-scripts
    if std::env::var_os("CARGO_CFG_WINDOWS").is_some() {
        let ico_path = "icons/icon-designer.microsoft.com-image-creator/ico/iced.ico";

        println!("cargo::rerun-if-changed=\"Cargo.toml\"");
        println!("cargo::rerun-if-changed=\"{ico_path}\"");

        let mut res = winresource::WindowsResource::new();
        res.set_icon(ico_path);
        res.compile().unwrap();
    }
}