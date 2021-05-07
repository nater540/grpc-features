#[cfg(target_os = "linux")]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

use structopt::StructOpt;
use feature_server::prelude::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let opt  = Opt::from_args();
  let data = Data::new(opt.clone()).await?;

  Ok(())
}
