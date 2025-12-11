use std::{
    error::Error,
    path::PathBuf,
    process::exit,
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = std::env::args().skip(1);
    let Some(cmd) = args.next() else {
        print_usage();
        exit(1);
    };

    match cmd.as_str() {
        "regen-protos" => regen_protos(),
        _ => {
            print_usage();
            exit(1);
        }
    }
}

fn regen_protos() -> Result<(), Box<dyn Error>> {
    let crate_dir = PathBuf::from("fuel-core-protobuf");
    let proto_dir = crate_dir.join("proto");
    let out_dir = crate_dir.join("src").join("generated");
    std::fs::create_dir_all(&out_dir)?;

    println!("Regenerating protobufs into {}", out_dir.display());

    tonic_prost_build::configure()
        .type_attribute(".", "#[derive(serde::Serialize,serde::Deserialize)]")
        .type_attribute(".", "#[allow(clippy::large_enum_variant)]")
        .out_dir(out_dir)
        .compile_protos(&[proto_dir.join("api.proto")], &[proto_dir])?;


    Ok(())
}

fn print_usage() {
    eprintln!("xtask commands:");
    eprintln!("  regen-protos   Regenerate protobuf bindings");
}
