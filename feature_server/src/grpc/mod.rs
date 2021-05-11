use tonic::{Request, Response, Status};
use futures_util::pin_mut;
use tokio::sync::RwLock;
use futures::Stream;
use std::sync::Arc;
use std::pin::Pin;

use grpc_api::prelude::*;
use grpc_api::server::*;
use crate::data::Data;
use crate::amqp::*;

type ResponseStream = Pin<Box<dyn Stream<Item = Result<StreamFeaturesResponse, Status>> + Send + Sync>>;

pub type FeaturesServer = grpc_api::server::FeaturesServer<FeatureServer>;

pub struct FeatureServer {
  pub data: Arc<Data>,
  pub amqp_rx: async_channel::Receiver<Message>
  // pub amqp_rx: RwLock<async_channel::Receiver<Message>>
}

impl FeatureServer {
  pub fn new(data: Data, amqp_rx: async_channel::Receiver<Message>) -> Self {
    let data = Arc::new(data);
    // let ampq_rx = RwLock::new(ampq_rx);

    FeatureServer {
      data,
      amqp_rx
    }
  }
}

#[tonic::async_trait]
impl Features for FeatureServer {
  type AllFeaturesStream = ResponseStream;

  async fn all_features(&self, request: Request<StreamFeaturesRequest>) -> Result<Response<Self::AllFeaturesStream>, Status> {
    use stream_features_response::FeatureResponse;
    info!("AllFeatures = {:?}", request);

    let amqp_rx = self.amqp_rx.clone();

    let output = async_stream::try_stream! {
      // TODO: Yield the initial set of features
      yield StreamFeaturesResponse {
        feature_response: Some(
          FeatureResponse::List(FeatureList {
            features: vec![]
          })
        )
      };

      // pin_mut!(amqp_rx);
      while let Ok(message) = amqp_rx.recv().await {
        info!("Received channel message, broadcasting...");
        yield StreamFeaturesResponse {
          feature_response: Some(
            FeatureResponse::Created(Feature {
              name: message.payload.test
            })
          )
        };
      }
    };

    Ok(Response::new(Box::pin(output) as Self::AllFeaturesStream))
  }
}
