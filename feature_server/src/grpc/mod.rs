use tonic::{transport::Server, Request, Response, Status, Streaming};
use futures::Stream;
use std::sync::Arc;
use std::pin::Pin;

use grpc_api::prelude::*;
use grpc_api::server::*;
use crate::data::Data;

type ResponseStream = Pin<Box<dyn Stream<Item = Result<StreamFeaturesResponse, Status>> + Send + Sync>>;

pub struct FeatureServer {
  pub data: Arc<Data>
}

// #[tonic::async_trait]
// impl Features for FeatureServer {
//   type AllFeaturesStream = ResponseStream;

//   async fn all_features(&self, request: Request<StreamFeaturesRequest>) -> Result<Response<Self::AllFeaturesStream>, Status> {
//     use stream_features_response::FeatureResponse;
//     debug!("AllFeatures = {:?}", request);

//     let output = async_stream::try_stream! {
//       // TODO: Yield the initial set of features
//       yield StreamFeaturesResponse {
//         feature_response: Some(
//           FeatureResponse::List(FeatureList {})
//         )
//       }
//     };

//     Ok(Response::new(Box::pin(output) as Self::AllFeaturesStream))
//   }
// }
