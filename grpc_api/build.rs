use std::path::PathBuf;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

  tonic_build::configure()
    .file_descriptor_set_path(out_dir.join("features_descriptor.bin"))
    .protoc_arg("--experimental_allow_proto3_optional")
    .build_server(true)
    .build_client(true)
    .compile(
      &[
        "proto/features/v1/service.proto",
        "proto/features/v1/feature.proto"
      ],
      &["proto/"]
    )?;


  Ok(())
}
