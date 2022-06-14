# hedge bot

Experimental hedge rebalancer bot.

## Features

- [ ] Rebalances portfolio to keep fixed percentages of assets
- [ ] Detects the best possible trades
- [ ] Cap the min trades amount to fight gas prices
- [ ] Config via CLI arguments. For now the configuration is done in `main.rs`

## Usage

### Ethereum Node
It is required to run your own `openethereum` node with loaded and unlocked private key.

### Set Config
Update `HTTP_ENDPOINT` to point the Ethereum Node and `POOL_ADDRESS` to be your dHedge pool address.