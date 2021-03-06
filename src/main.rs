use dht::*;
use env_logger::Builder;
use log::*;
use std::collections::HashMap;

const HTTP_ENDPOINT: &str = "http://161.35.205.60:8545";
const POOL_ADDRESS: &str = "53523de8a90053ddb1d330499d3dc080b909edb9";

#[tokio::main]
async fn main() -> Result<(), ()> {
    Builder::new().parse_filters("INFO").init();

    info!("Hedge Trader v0.1.0");
    let mut expected_shares = HashMap::new();
    expected_shares.insert("sBTC".to_string(), 0.10);
    expected_shares.insert("sETH".to_string(), 0.20);
    expected_shares.insert("sUSD".to_string(), 0.20);
    expected_shares.insert("sBNB".to_string(), 0.10);
    expected_shares.insert("sLTC".to_string(), 0.10);
    expected_shares.insert("sDEFI".to_string(), 0.10);
    expected_shares.insert("iLINK".to_string(), 0.20);

    let symbols = vec!["sBTC", "sETH", "sUSD", "sBNB", "sLTC", "sDEFI", "iLINK"];

    let mut hedge = Hedge::new(HTTP_ENDPOINT, POOL_ADDRESS).await;
    let all_assets: HashMap<Symbol, Asset> = hedge.get_fund_composition().await;
    let mut assets = HashMap::new();
    for symbol in symbols.clone() {
        let asset = all_assets.get(symbol).unwrap();
        assets.insert(symbol.to_string(), asset.clone());
    }
    assert_eq!(symbols.len(), assets.len());

    let pool = Pool::new(expected_shares, assets, 3.0);
    pool.print_status();

    if pool.balanced() {
        info!("Pool is already balanced.");
    } else {
        info!("Rebalancing");
        let swaps = pool.rebalance_plan();
        hedge.rebalance(swaps).await;
        info!("Rebalancing done!");
    }
    info!("Done");
    Ok(())
}