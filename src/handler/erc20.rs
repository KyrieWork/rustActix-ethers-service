use ethers::{
    prelude::abigen,
    providers::{Http, Provider},
    types::{Address, U256},
};
use std::sync::Arc;
const RPC_URL: &str = "https://eth-mainnet.g.alchemy.com/v2/8GWun9gYRI-6m14LpcXyX1qEgISt9Tpn";

abigen!(
    IERC20,
    r#"[
    function totalSupply() external view returns (uint256)
    function balanceOf(address account) external view returns (uint256)
    function transfer(address recipient, uint256 amount) external returns (bool)
    function allowance(address owner, address spender) external view returns (uint256)
    function approve(address spender, uint256 amount) external returns (bool)
    function transferFrom( address sender, address recipient, uint256 amount) external returns (bool)
    event Transfer(address indexed from, address indexed to, uint256 value)
    event Approval(address indexed owner, address indexed spender, uint256 value)
]"#,
);
pub async fn erc20_contract(address: &str) -> IERC20<ethers::providers::Provider<Http>> {
    let provider = Provider::<Http>::try_from(RPC_URL).unwrap();
    let client = Arc::new(provider);
    let addr: Address = address.parse().unwrap();
    let contract = IERC20::new(addr, client);
    contract
}

pub async fn erc20_balance(contract_addr: &str, user_addr: &str) -> Result<U256, String> {
    let contract = erc20_contract(contract_addr).await;
    let account: Address = user_addr.parse().unwrap();
    if let Ok(balance) = contract.balance_of(account).call().await {
        Ok(balance)
    } else {
        Err(String::from("contract.balance_of error message"))
    }
}

pub async fn erc20_allowance(
    contract_addr: &str,
    owner: &str,
    spender: &str,
) -> Result<U256, String> {
    let contract = erc20_contract(contract_addr).await;
    let owner: Address = owner.parse().unwrap();
    let spender: Address = spender.parse().unwrap();
    if let Ok(balance) = contract.allowance(owner, spender).call().await {
        Ok(balance)
    } else {
        Err(String::from("contract.balance_of error message"))
    }
}
