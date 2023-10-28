use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut builder = tonic_build::configure()
        .build_client(false)
        .build_server(false)
        .protoc_arg("--experimental_allow_proto3_optional");

    if env::var("CARGO_FEATURE_CLIENT_PROTO").is_ok() {
        builder = builder.build_client(true);
    }
    if env::var("CARGO_FEATURE_SERVER_PROTO").is_ok() {
        builder = builder.build_server(true);
    }

    builder.compile(&["proto/daktilo.proto"], &["proto"])?;

    Ok(())
}
