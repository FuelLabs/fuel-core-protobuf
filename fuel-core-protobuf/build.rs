fn main() {
    // Keep the build script a no-op for consumers; regeneration happens via `cargo xtask`
    // and the committed `src/generated` files are used at build time.
    println!("cargo:rerun-if-changed=src/generated/blockaggregator.rs");
}
