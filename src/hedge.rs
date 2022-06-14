use log::*;
use std::collections::HashMap;

use web3::contract::{Contract, Options};
use web3::types::{Address, U256};

type Transport = web3::transports::batch::Batch<web3::transports::http::Http>;

pub struct Hedge {
    manager: Address,
    contract: Contract<Transport>,
    web3: web3::Web3<Transport>,
}

impl Hedge {
    pub async fn new(endpoint: &str, pool_address: &str) -> Hedge {
        let pool_address: Address = pool_address.parse().unwrap();
        let http = web3::transports::Http::new(endpoint).unwrap();
        let web3 = web3::Web3::new(web3::transports::Batch::new(http));

        let contract =
            Contract::from_json(web3.eth(), pool_address, include_bytes!("hedge.abi")).unwrap();
        let accounts = web3.personal().list_accounts();
        web3.transport().submit_batch().await.unwrap();

        let manager = accounts.await.unwrap()[0];
        info!("Using manager account: {}", manager);
        Hedge {
            manager,
            contract,
            web3,
        }
    }

    pub async fn get_fund_composition(&mut self)  {

    }

    pub async fn rebalance(&mut self) {
      info!("Reading from blockchain. Calling getFundComposition at {}", self.contract.address());

      let data = self.contract.query("getFundComposition", (), None, Options::default(), None);
      self.submit_batch().await;

      let (symbols, balances, rates): (Vec<[u8; 32]>, Vec<U256>, Vec<U256>) = data.await.unwrap();
      // let mut assets: HashMap<Symbol, Asset> = HashMap::new();
    }

    pub async fn submit_batch(&mut self) {
      self.web3.transport().submit_batch().await.unwrap();
    }

    pub async fn get_nonce(&mut self) -> U256 {
      let nonce = self.web3.eth().transaction_count(self.manager, None);
      self.submit_batch().await;
      nonce.await.unwrap()
    }
}
