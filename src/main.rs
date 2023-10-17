use web3::{
  futures::{future, StreamExt},
  types::BlockHeader,
  Error,
};

#[tokio::main]
async fn main() -> web3::Result<()> {
  let ws = web3::transports::WebSocket::new(
    "wss://mainnet.infura.io/ws/v3/c60b0bb42f8a4c6481ecd229eddaca27",
  )
  .await?;
  let web3 = web3::Web3::new(ws.clone());
  let mut sub = web3.eth_subscribe().subscribe_new_heads().await?;

  println!("Got subscription id: {:?}", sub.id());

  (&mut sub)
    .take_while(|x| {
      future::ready(match x {
        Ok(_x) => true,
        Err(_err) => false,
      })
    })
    .for_each(|x: Result<BlockHeader, web3::Error>| {
      process_event(x);
      future::ready(())
    })
    .await;

  sub.unsubscribe().await?;

  Ok(())
}

fn process_event(data: Result<BlockHeader, Error>) {
  println!("Got: {:?}", data);
}
