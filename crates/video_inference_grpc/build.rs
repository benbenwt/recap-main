//! This build script generates the gRPC client code.

#[allow(unsafe_code)]
fn set_protoc_env() {
    unsafe {
        std::env::set_var(
            "PROTOC",
            if cfg!(target_os = "windows") {
                "protoc.exe"
            } else {
                "protoc"
            },
        );
    };
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Ensure protoc is discoverable for prost/tonic builds
    set_protoc_env();
    tonic_prost_build::configure()
        .build_client(cfg!(feature = "client"))
        .build_server(cfg!(feature = "server"))
        .include_file("mod.rs")
        .compile_protos(
            &[
                "./../../protos/video_inference.proto",
                "./../../protos/shared.proto",
            ],
            &["./../../protos/"],
        )?;
    Ok(())
}
