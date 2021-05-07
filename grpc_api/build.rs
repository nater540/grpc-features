fn main() -> Result<(), Box<dyn std::error::Error>> {
  tonic_build::configure()
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
