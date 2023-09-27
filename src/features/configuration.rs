#![allow(deprecated)]
use futures::channel::oneshot;
use futures::future::pending;
use libp2p::identity;
use libp2p::identity::Keypair;
use libp2p::kad::Mode;
use libp2p::multiaddr::Protocol;
use parking_lot::Mutex;
use std::sync::Arc;
use subspace_networking::{Config, Node, PeerInfoProvider, PieceByIndexRequestHandler};

pub async fn configure_dsn(bootstrap_address: String, protocol_prefix: &'static str) -> Node {
    let keypair = Keypair::generate_ed25519();

    let default_config = Config::new(
        protocol_prefix.to_string(),
        keypair,
        (),
        Some(PeerInfoProvider::Client),
    );

    let config_1 = Config {
        listen_on: vec!["/ip4/0.0.0.0/tcp/44001".parse().unwrap()],
        allow_non_global_addresses_in_dht: false,
        kademlia_mode: Some(Mode::Client),
        request_response_protocols: vec![PieceByIndexRequestHandler::create(|_, _| async { None })],
        bootstrap_addresses: vec![bootstrap_address.parse().unwrap()],
        ..default_config
    };
    let (node, mut node_runner_1) = subspace_networking::construct(config_1).unwrap();

    let (node_address_sender, node_address_receiver) = oneshot::channel();
    let on_new_listener_handler = node.on_new_listener(Arc::new({
        let node_address_sender = Mutex::new(Some(node_address_sender));

        move |address| {
            if matches!(address.iter().next(), Some(Protocol::Ip4(_))) {
                if let Some(node_address_sender) = node_address_sender.lock().take() {
                    node_address_sender.send(address.clone()).unwrap();
                }
            }
        }
    }));

    tokio::spawn({
        let node = node.clone();
        async move {
            let _ = node.bootstrap().await;

            pending::<()>().await;
        }
    });

    tokio::spawn(async move {
        node_runner_1.run().await;
    });

    // Wait for first node to know its address
    let node_addr = node_address_receiver.await.unwrap();
    drop(on_new_listener_handler);

    println!("Node 1 ID is {}", node.id());
    println!("Node 1 address {}", node_addr);

    node
}
