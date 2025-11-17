# fuel-core-protobuf
Protobuf Definitions for the Fuel Core RPC

## Regenerating

The generated Rust bindings live in `src/generated/blockaggregator.rs` so
consumers of the crate do not need `protoc` installed. To refresh the generated
code (for CI or local development), run:

```bash
cargo xtask regen-protos
git diff src/generated/blockaggregator.rs
```

Then check in any changes to the `src/generated` directory.
