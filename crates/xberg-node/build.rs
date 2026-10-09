fn main() {
    napi_build::setup();

    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("macos") {
        // ~keep ld64 preserves Mach-O LINKEDIT alignment; rustc stripping does not (rust-lang/rust#157750).
        println!("cargo:rustc-link-arg=-Wl,-x");
    }
}
