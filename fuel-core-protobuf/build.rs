use std::{env, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=proto/api.proto");

    // Only regenerate the protobuf bindings when explicitly requested. This keeps
    // consumers from needing `protoc` while still letting CI/users refresh the
    // checked-in code with `PROTO_REGENERATE=1 cargo build`.
    if env::var_os("PROTO_REGENERATE").is_none() {
        return Ok(());
    }

    let out_dir =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR")?).join("src").join("generated");
    std::fs::create_dir_all(&out_dir)?;

    tonic_prost_build::configure()
        .type_attribute(".", "#[derive(serde::Serialize,serde::Deserialize)]")
        .type_attribute(".", "#[allow(clippy::large_enum_variant)]")
        .out_dir(out_dir)
        .compile_protos(&["proto/api.proto"], &["proto/"])?;

    Ok(())
}
