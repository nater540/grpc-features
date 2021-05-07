pub mod proto {
  pub mod features {
    pub mod v1 {
      tonic::include_proto!("features.v1");
    }
  }
}

pub mod server {
  pub use crate::proto::features::v1::features_server::*;
}

pub mod prelude {
  pub use crate::proto::features::v1::*;
}
