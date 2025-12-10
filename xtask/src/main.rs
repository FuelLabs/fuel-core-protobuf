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
        .out_dir(&out_dir)
        .compile_protos(&[proto_dir.join("api.proto")], &[proto_dir])?;

    patch_block_response_payload(out_dir.join("blockaggregator.rs"))?;

    Ok(())
}

fn patch_block_response_payload(path: PathBuf) -> Result<(), Box<dyn Error>> {
    const ORIGINAL: &str = r#"#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::large_enum_variant)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockResponse {
    #[prost(uint32, tag = "1")]
    pub height: u32,
    #[prost(oneof = "block_response::Payload", tags = "2, 3")]
    pub payload: ::core::option::Option<block_response::Payload>,
}
/// Nested message and enum types in `BlockResponse`.
pub mod block_response {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::large_enum_variant)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Payload {
        #[prost(message, tag = "2")]
        Literal(super::Block),
        #[prost(message, tag = "3")]
        Remote(super::RemoteBlockResponse),
    }
}
"#;

    const UPDATED: &str = r#"#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::large_enum_variant)]
#[derive(Clone, PartialEq, Debug, Default)]
pub struct BlockResponse {
    pub height: u32,
    pub payload: ::core::option::Option<block_response::Payload>,
}
/// Nested message and enum types in `BlockResponse`.
pub mod block_response {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::large_enum_variant)]
    #[derive(Clone, PartialEq, Debug)]
    pub enum Payload {
        Literal(::std::sync::Arc<super::Block>),
        Remote(super::RemoteBlockResponse),
    }

    impl Payload {
        pub fn encode(&self, buf: &mut impl ::prost::bytes::BufMut) {
            match self {
                Payload::Literal(value) => {
                    ::prost::encoding::message::encode(2, value.as_ref(), buf);
                }
                Payload::Remote(value) => {
                    ::prost::encoding::message::encode(3, value, buf);
                }
            }
        }

        pub fn merge(
            field: &mut Option<Self>,
            tag: u32,
            wire_type: ::prost::encoding::wire_type::WireType,
            buf: &mut impl ::prost::bytes::Buf,
            ctx: ::prost::encoding::DecodeContext,
        ) -> Result<(), ::prost::DecodeError> {
            match tag {
                2 => {
                    if let Some(Payload::Literal(value)) = field {
                        ::prost::encoding::message::merge(
                            wire_type,
                            ::std::sync::Arc::make_mut(value),
                            buf,
                            ctx,
                        )
                    } else {
                        let mut value = ::core::default::Default::default();
                        ::prost::encoding::message::merge(wire_type, &mut value, buf, ctx)?;
                        *field = Some(Payload::Literal(::std::sync::Arc::new(value)));
                        Ok(())
                    }
                }
                3 => {
                    if let Some(Payload::Remote(value)) = field {
                        ::prost::encoding::message::merge(wire_type, value, buf, ctx)
                    } else {
                        let mut value = ::core::default::Default::default();
                        ::prost::encoding::message::merge(wire_type, &mut value, buf, ctx)?;
                        *field = Some(Payload::Remote(value));
                        Ok(())
                    }
                }
                _ => unreachable!(concat!("invalid ", stringify!(Payload), " tag: {}"), tag),
            }
        }

        #[inline]
        pub fn encoded_len(&self) -> usize {
            match self {
                Payload::Literal(value) => {
                    ::prost::encoding::message::encoded_len(2, value.as_ref())
                }
                Payload::Remote(value) => ::prost::encoding::message::encoded_len(3, value),
            }
        }
    }
}

#[allow(clippy::large_enum_variant)]
impl ::prost::Message for BlockResponse {
    fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
        if self.height != 0 {
            ::prost::encoding::uint32::encode(1, &self.height, buf);
        }
        if let Some(ref payload) = self.payload {
            payload.encode(buf);
        }
    }

    fn merge_field(
        &mut self,
        tag: u32,
        wire_type: ::prost::encoding::wire_type::WireType,
        buf: &mut impl ::prost::bytes::Buf,
        ctx: ::prost::encoding::DecodeContext,
    ) -> ::core::result::Result<(), ::prost::DecodeError> {
        match tag {
            1 => ::prost::encoding::uint32::merge(wire_type, &mut self.height, buf, ctx),
            2 | 3 => {
                block_response::Payload::merge(&mut self.payload, tag, wire_type, buf, ctx)
            }
            _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
        }
    }

    fn encoded_len(&self) -> usize {
        let mut len = 0;
        if self.height != 0 {
            len += ::prost::encoding::uint32::encoded_len(1, &self.height);
        }
        if let Some(ref payload) = self.payload {
            len += payload.encoded_len();
        }
        len
    }

    fn clear(&mut self) {
        self.height = 0;
        self.payload = None;
    }
}
"#;

    let mut contents = std::fs::read_to_string(&path)?;

    if contents.contains(ORIGINAL) {
        contents = contents.replacen(ORIGINAL, UPDATED, 1);
    } else {
        return Err(format!(
            "expected block_response::Payload definition not found in {}",
            path.display()
        )
        .into());
    }

    std::fs::write(path, contents)?;

    Ok(())
}

fn print_usage() {
    eprintln!("xtask commands:");
    eprintln!("  regen-protos   Regenerate protobuf bindings");
}
