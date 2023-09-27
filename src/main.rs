use ethers::prelude::*;
use ethers::{
  contract::{abigen, Contract},
  core::types::ValueOrArray,
  providers::{Provider, StreamExt, Ws},
};
use std::{error::Error, sync::Arc};

abigen!(
  AggregatorInterface,
  r#"[
        event AnswerUpdated(int256 indexed current, uint256 indexed roundId, uint256 updatedAt)
    ]"#,
);

const PRICE_FEED_1: &str = "0x7de93682b9b5d80d45cd371f7a14f74d49b0914c";
const PRICE_FEED_2: &str = "0x0f00392fcb466c0e4e4310d81b941e07b4d5a079";
const PRICE_FEED_3: &str = "0xebf67ab8cff336d3f609127e8bbf8bd6dd93cd81";

/// Subscribe to a typed event stream without requiring a `Contract` instance.
/// In this example we subscribe Chainlink price feeds and filter out them
/// by address.
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  let client = get_client().await;
  let client = Arc::new(client);

  // Build an Event by type. We are not tied to a contract instance. We use builder functions to
  // refine the event filter
  let event = Contract::event_of_type::<AnswerUpdatedFilter>(client)
    .from_block(16022082)
    .address(ValueOrArray::Array(vec![
      PRICE_FEED_1.parse()?,
      PRICE_FEED_2.parse()?,
      PRICE_FEED_3.parse()?,
    ]));

  let mut stream = event.subscribe_with_meta().await?.take(2);

  // Note that `log` has type AnswerUpdatedFilter
  while let Some(Ok((log, meta))) = stream.next().await {
    println!("{log:?}");
    println!("{meta:?}")
  }

  Ok(())
}

async fn get_client() -> Provider<Ws> {
  Provider::<Ws>::connect("wss://mainnet.infura.io/ws/v3/c60b0bb42f8a4c6481ecd229eddaca27")
    .await
    .unwrap()
}

async fn get_events() -> Result<(), Box<dyn std::error::Error>> {
  // Abi
  abigen!(ERC20, "./abi/weth.json");

  // const RPC_URL: &str = "https://mainnet.infura.io/v3/682677fe6f69476184a0c168aff207cf";
  const WS_URL: &str = "wss://mainnet.infura.io/ws/v3/682677fe6f69476184a0c168aff207cf";
  const WETH_ADDRESS: &str = "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2";
  // let provider = Provider::<Http>::try_from(RPC_URL)?;
  let provider = Provider::<Ws>::connect(WS_URL).await?;

  let client = Arc::new(provider);
  let address: Address = WETH_ADDRESS.parse()?;

  let contract = ERC20::new(address, client);

  if let Ok(total_supply) = contract.total_supply().call().await {
    println!("WETH total supply is {total_supply:?}");
  }

  let events = contract.events().from_block(16022082);
  let mut stream = events.stream().await?.take(1);

  while let Some(Ok(evt)) = stream.next().await {
    match evt {
      ERC20Events::ApprovalFilter(f) => println!("{f:?}"),
      ERC20Events::TransferFilter(f) => println!("{f:?}"),
      ERC20Events::DepositFilter(f) => println!("{f:?}"),
      ERC20Events::WithdrawalFilter(f) => println!("{f:?}"),
    }
  }

  Ok(())
}
