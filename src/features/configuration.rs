use futures::channel::oneshot;
use libp2p::multiaddr::Protocol;
use libp2p::{identity};
use parking_lot::Mutex;
use std::sync::Arc;
use subspace_networking::{BootstrappedNetworkingParameters, Config, Node, PieceAnnouncementRequestHandler, PieceByHashRequestHandler, VoidProviderStorage};

pub async fn configure_dsn(bootstrap_address: String, protocol_prefix: &'static str) -> Node {
    let ed25519_keypair = identity::ed25519::Keypair::generate();

    let keypair = identity::Keypair::Ed25519(ed25519_keypair);
    let default_config = Config::new(protocol_prefix.to_string(), keypair, VoidProviderStorage);

    let config_1 = Config::<VoidProviderStorage> {
        listen_on: vec!["/ip4/0.0.0.0/tcp/40001".parse().unwrap()],
        allow_non_global_addresses_in_dht: true,
        networking_parameters_registry: BootstrappedNetworkingParameters::new(vec![
            bootstrap_address.parse().unwrap(),
        ])
        .boxed(),
        request_response_protocols: vec![
            PieceByHashRequestHandler::create(|_,_| async { None }),
            PieceAnnouncementRequestHandler::create(|_, _| async { None }),
        ],
    //    protocol_prefix,
        ..default_config
    };
    let (node, mut node_runner_1) = subspace_networking::create(config_1).unwrap();

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
