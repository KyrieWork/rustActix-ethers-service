use ethers::{
    providers::{Http, Middleware, Provider},
    types::{Address, U256},
};
use std::convert::TryFrom;

// const RPC_URL: &str = "https://eth-mainnet.g.alchemy.com/v2/8GWun9gYRI-6m14LpcXyX1qEgISt9Tpn";
const RPC_URL: &str = "http://127.0.0.1:8545/";

pub async fn eth_balance(address: Address) -> Result<U256, Box<dyn std::error::Error>> {
    let provider = Provider::<Http>::try_from(RPC_URL)?;
    let balance = provider.get_balance(address, None).await?;
    Ok(balance)
}
pub fn eth_format(amount: U256) -> String {
    ethers::utils::format_units(amount, 18).unwrap()
}
pub fn eth_parse(amount: String) -> U256 {
    U256::from(ethers::utils::parse_units(amount, 18).unwrap())
}
