use std::{
    env,
    error::Error,
    process::{exit, Command},
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args().skip(1);
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
    let status = Command::new("cargo")
        .env("PROTO_REGENERATE", "1")
        .args([
            "build",
            "--manifest-path",
            "fuel-core-protobuf/Cargo.toml",
            "--package",
            "fuel-core-protobuf",
        ])
        .status()?;

    if !status.success() {
        return Err("cargo build failed while regenerating protos".into());
    }

    Ok(())
}

fn print_usage() {
    eprintln!("xtask commands:");
    eprintln!("  regen-protos   Regenerate protobuf bindings");
}
