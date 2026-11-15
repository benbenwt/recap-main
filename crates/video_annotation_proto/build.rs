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
    let mut config = prost_build::Config::new();

    println!("cargo:rerun-if-changed=./../../protos/video_annotation.proto");

    #[cfg(feature = "serialize")]
    config.type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]");

    config.include_file("mod.rs").compile_protos(
        &[
            "./../../protos/video_annotation.proto",
            "./../../protos/shared.proto",
        ],
        &["./../../protos/"],
    )?;
    Ok(())
}
