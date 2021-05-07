use lapin::{Channel, Connection, ConnectionProperties};
use tokio::sync::RwLock;
use tokio_amqp::*;

#[derive(Debug)]
pub struct RabbitMQ {
  pub conn: RwLock<Connection> // TODO: Might not be necessary to keep this in a RwLock
}

impl RabbitMQ {
  pub async fn new(amqp_path: &str) -> anyhow::Result<RabbitMQ> {
    let conn = Connection::connect(amqp_path, ConnectionProperties::default().with_tokio()).await?;

    Ok(RabbitMQ { conn: RwLock::new(conn) })
  }
}
