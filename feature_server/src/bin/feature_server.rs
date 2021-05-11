#[cfg(target_os = "linux")]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

use structopt::StructOpt;
use feature_server::prelude::*;
use feature_server::amqp::*;
use feature_server::grpc::*;

use tonic::transport::Server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  pretty_env_logger::init();

  let opt  = Opt::from_args();
  let data = Data::new(opt.clone()).await?;

  let wabbit = RabbitMQ::new(&data.amqp_path).await?;
  let (rx, listen_handle) = wabbit.start_listener(&opt.amqp_queue).await?;

  let reflection_service = tonic_reflection::server::Builder::configure()
    .register_encoded_file_descriptor_set(grpc_api::proto::features::v1::FILE_DESCRIPTOR_SET)
    .build()?;

  let addr = "127.0.0.1:9000".parse().unwrap();
  Server::builder()
    .add_service(reflection_service)
    .add_service(FeaturesServer::new(FeatureServer::new(data.clone(), rx.clone())))
    .serve(addr)
    .await?;

  Ok(())
}
