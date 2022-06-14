use ht::*;
use env_logger::Builder;
use log::*;
use std::collections::HashMap;

async fn main() -> Result<(), ()> {
  Builder::new().parse_filters("INFO").init();

  info!("Hedge Bot v0.1.0");
  let mut expected_shares = HashMap::new();
  expected_shares.insert("sBTC".to_string(), 0.10);
  expected_shares.insert("sETH".to_string(), 0.20);

  let symbols = vec!["sBTC", "sETH"];

  Ok(())
}