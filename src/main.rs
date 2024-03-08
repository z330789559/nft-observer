use sui_sdk::SuiClientBuilder;
use sui_sdk::rpc_types::EventFilter;
use sui_sdk::types::Identifier;
use futures::stream::StreamExt;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Sui testnet -- https://fullnode.testnet.sui.io:443
    let ws: sui_sdk::SuiClient = SuiClientBuilder::default()
    .ws_url("wss://rpc.testnet.sui.io:443")
    .build("https://fullnode.testnet.sui.io:443")
    .await?;
println!("WS version {:?}", ws.api_version());

// :MoveFunction {
//     package: "0x2ccd199d8848696534ee129dc2281785535de330ea3d0e1d3f3c0baf8b0691a1".parse()?,
//     module: Some("nft_first".to_owned()),
//     function: Some("sword_transfer".to_owned()),
// }
let subs= ws.available_subscriptions();
println!("{:?}",subs);

let mut subscribe = ws
    .event_api()
    .subscribe_event(
        EventFilter::All(vec![])
    )
    .await?;

loop {
    println!("{:?}", subscribe.next().await);
}

}