use lazy_static::lazy_static;
use serde::Deserialize;

lazy_static! {
  pub static ref ENV_CONFIG: EnvConfig = {
    dotenvy::dotenv().ok();
    envy::from_env().expect("Failed to load environment variables")
  };
}

fn default_host() -> String {
  "127.0.0.1".to_string()
}

fn default_port() -> u16 {
  3000
}

fn default_mongodb_uri() -> String {
  "mongodb://root:example@127.0.0.1:27017".to_string()
}

fn default_mongodb_name() -> String {
  "myproman".to_string()
}

#[derive(Deserialize, Debug)]
pub struct EnvConfig {
  #[serde(default = "default_host")]
  pub server_host: String,

  #[serde(default = "default_port")]
  pub server_port: u16,

  #[serde(default = "default_mongodb_uri")]
  pub mongodb_uri: String,

  #[serde(default = "default_mongodb_name")]
  pub mongodb_name: String,
}
