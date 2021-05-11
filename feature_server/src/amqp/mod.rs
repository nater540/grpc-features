use lapin::{Channel, Connection, ConnectionProperties, options::{BasicAckOptions, BasicConsumeOptions}, types::FieldTable};
use tokio::{sync::RwLock, sync::broadcast, task::JoinHandle};
use tokio_amqp::*;
use anyhow::Result;

mod message;
pub(crate) use message::*;

#[derive(Debug)]
pub struct RabbitMQ {
  pub conn: RwLock<Connection>,
  pub channel: RwLock<Channel>
}

impl RabbitMQ {
  pub async fn new(amqp_path: &str) -> Result<RabbitMQ> {
    let conn = Connection::connect(amqp_path, ConnectionProperties::default().with_tokio()).await?;
    let channel = conn.create_channel().await?;
    Ok(RabbitMQ { conn: RwLock::new(conn), channel: RwLock::new(channel) })
  }

  pub async fn start_listener(&self, amqp_queue: &str) -> Result<(broadcast::Sender<Message>, JoinHandle<Result<(), anyhow::Error>>)> {
    let channel = self.channel.read().await;
    let mut consumer = channel.basic_consume(
      amqp_queue,
      "",
      BasicConsumeOptions::default(),
      FieldTable::default()
    ).await?;

    //let (tx, rx) = async_channel::unbounded::<Message>();
    let (tx, rx) = broadcast::channel::<Message>(69);

    let listen_tx = tx.clone();
    let listen_task = tokio::spawn(async move {

      loop {
        use futures::stream::StreamExt;
        use std::convert::TryFrom;

        while let Some(delivery) = consumer.next().await {
          if let Ok((_channel, delivery)) = delivery {
            let message = Message::try_from(&delivery)?;
            info!("Received message: {}", message.payload.test.clone());
            listen_tx.send(message)?;

            delivery.ack(BasicAckOptions::default()).await?;
          }
        }
      }

      Ok::<(), anyhow::Error>(()) // Type annotation to appease the rust compiler :)
    });

    Ok((tx, listen_task))
  }
}
