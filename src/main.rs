use clap::Parser;
use nostr::util::nips::nip19::FromBech32;
use nostr::util::time::timestamp;
use nostr::Keys;
use nostr::{Kind, SubscriptionFilter};
use nostr_sdk::{Client, RelayPoolNotifications, Result};

pub mod types;

/// cli arguments
#[derive(Parser, Debug)]
#[clap(author, version, about)]
/// Mostro P2P cli client
struct Arguments {
    list: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();
    // TODO: handle arguments
    // let args = Arguments::parse();

    // mostro pubkey
    let pubkey = "npub1m0str0n64lfulw5j6arrak75uvajj60kr024f5m6c4hsxtsnx4dqpd9ape";
    let mostro_keys = nostr::key::XOnlyPublicKey::from_bech32(pubkey)?;

    // Generate new keys
    let my_keys: Keys = Client::generate_keys();
    // Create new client
    let client = Client::new(&my_keys);

    // Add relays
    // client.add_relay("wss://relay.damus.io", None).await?;
    // client.add_relay("wss://nostr.fly.dev", None).await?;
    client.add_relay("wss://nostr.zebedee.cloud", None).await?;
    // client
    //     .add_relay("wss://relay.minds.com/nostr/v1/ws", None)
    //     .await?;
    // client.add_relay("wss://nostr.fly.dev", None).await?;
    // client.add_relay("wss://nostr.openchain.fr", None).await?;

    // Connect to relays and keep connection alive
    client.connect().await?;

    let subscription = SubscriptionFilter::new()
        .author(mostro_keys)
        .since(timestamp());

    client.subscribe(vec![subscription]).await?;

    // Handle notifications
    loop {
        let mut notifications = client.notifications();
        while let Ok(notification) = notifications.recv().await {
            if let RelayPoolNotifications::ReceivedEvent(event) = notification {
                if let Kind::Custom(kind) = event.kind {
                    if kind == 30000 {
                        let order = types::Order::from_json(&event.content)?;
                        println!("Event id: {}", event.id);
                        println!("Event kind: {}", kind);
                        println!("Order: {:#?}", order);
                    }
                }
            }
        }
    }
}