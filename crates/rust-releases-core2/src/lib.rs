pub struct Release {
    toolchain: rust_toolchain::Toolchain,
}

#[test]
fn test() {
    let release = Release {
        toolchain: rust_toolchain::RustupToolchain::active().unwrap().into(),
    };
}
