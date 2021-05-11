use serde::Deserialize;
use std::convert::TryFrom;

use lapin::{
  types::FieldTable,
  message::Delivery
};

#[derive(Debug, Clone)]
pub struct Message {
  pub headers: FieldTable,
  pub payload: Payload
}

#[derive(Debug, Clone, Deserialize)]
pub struct Payload {
  pub version: PayloadVersion,
  pub test: String
}

#[derive(Debug, Clone, Deserialize)]
pub enum PayloadVersion {
  V1
}

impl TryFrom<&Delivery> for Message {
  type Error = anyhow::Error;

  fn try_from(delivery: &Delivery) -> anyhow::Result<Self, Self::Error> {
    let mut headers = FieldTable::default();
    if let Some(props) = delivery.properties.headers() {
      headers = props.clone();
    }

    let data = std::str::from_utf8(&delivery.data)?;
    let payload: Payload = serde_json::from_str(data)?;
    Ok(Message { headers, payload })
  }
}
