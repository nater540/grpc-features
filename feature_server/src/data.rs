use std::ops::Deref;
use std::sync::Arc;
use anyhow::Result;

use crate::app::Opt;
use crate::amqp::RabbitMQ;

#[derive(Clone, Debug)]
pub struct Data {
  inner: Arc<DataInner>
}

impl Deref for Data {
  type Target = DataInner;
  fn deref(&self) -> &Self::Target {
    &self.inner
  }
}

#[derive(Clone, Debug)]
pub struct DataInner {
  pub amqp: Arc<RabbitMQ>,
  pub db_path: String,
  pub amqp_path: String,
  pub amqp_queue: String,
  pub env: String,
  pub server_pid: u32
}

impl Data {
  pub async fn new(opts: Opt) -> Result<Data> {
    let db_name = if let Some(name) = opts.db_name {
      name
    } else {
      format!("features_{}", opts.env)
    };

    // Build our db connection string
    let db_path = format!(
      "postgres://{}:{}@{}:{}/{}",
      opts.db_username,
      opts.db_password,
      opts.db_host,
      opts.db_port,
      db_name
    );

    // Build our amqp connection string
    let amqp_path = format!(
      "amqp://{}:{}@{}:{}/{}",
      opts.amqp_username,
      opts.amqp_password,
      opts.amqp_host,
      opts.amqp_port,
      opts.amqp_vhost
    );

    let amqp = Arc::new(RabbitMQ::new(&amqp_path).await?);

    let server_pid   = std::process::id();
    let amqp_queue = opts.amqp_queue.clone();
    let env        = opts.env.clone();

    let inner = DataInner {
      amqp: amqp.clone(),
      env,
      db_path,
      amqp_path,
      amqp_queue,
      server_pid
    };

    // Make it so number one!
    Ok(Data { inner: Arc::new(inner) })
  }

  /// Whether or not the server is running in production mode.
  pub fn production(&self) -> bool {
    self.env == "production"
  }

  /// Whether or not the server is running in staging mode.
  pub fn staging(&self) -> bool {
    self.env == "staging"
  }

  /// Whether or not the server is running in development mode.
  pub fn development(&self) -> bool {
    self.env == "development"
  }
}
