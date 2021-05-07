#[macro_use]
extern crate log;

pub mod app;
pub mod amqp;
pub mod data;
pub mod grpc;

pub mod prelude {
  pub use crate::app::Opt;
  pub use crate::data::Data;
}
