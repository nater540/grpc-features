use structopt::StructOpt;

const POSSIBLE_ENV: [&str; 3] = ["development", "staging", "production"];

#[derive(Debug, Default, Clone, StructOpt)]
#[structopt(name = "feature-server", about = "gRPC server for handling feature streams")]
pub struct Opt {
  #[structopt(long, env = "FEATURES_ENV", default_value = "development", possible_values = &POSSIBLE_ENV)]
  pub env: String,

  /// Database name to use. Defaults to `features_{ENV}`.
  #[structopt(long, env = "FEATURES_DB_NAME")]
  pub db_name: Option<String>,

  /// PostgreSQL server host.
  #[structopt(long, env = "FEATURES_DB_HOST", default_value = "localhost")]
  pub db_host: String,

  /// PostgreSQL server port.
  #[structopt(long, env = "FEATURES_DB_PORT", default_value = "5432")]
  pub db_port: String,

  /// PostgreSQL server username.
  #[structopt(long, env = "FEATURES_DB_USERNAME", default_value = "features")]
  pub db_username: String,

  /// PostgreSQL server password.
  #[structopt(long, env = "FEATURES_DB_PASSWORD", default_value = "SuperKawaii#1")]
  pub db_password: String,

  /// AMQP (RabbitMQ) server host.
  #[structopt(long, env = "AMQP_HOST", default_value = "localhost")]
  pub amqp_host: String,

  /// AMQP (RabbitMQ) server port.
  #[structopt(long, env = "AMQP_PORT", default_value = "5672")]
  pub amqp_port: String,

  /// AMQP (RabbitMQ) server username.
  #[structopt(long, env = "AMQP_USERNAME", default_value = "wabbit")]
  pub amqp_username: String,

  /// AMQP (RabbitMQ) server password.
  #[structopt(long, env = "AMQP_PASSWORD", default_value = "password")]
  pub amqp_password: String,

  /// AMQP (RabbitMQ) server virtual host.
  #[structopt(long, env = "AMQP_VHOST", default_value = "features")]
  pub amqp_vhost: String,

  /// AMQP (RabbitMQ) server queue name.
  #[structopt(long, env = "AMQP_QUEUE", default_value = "features-broadcast.work")]
  pub amqp_queue: String
}
