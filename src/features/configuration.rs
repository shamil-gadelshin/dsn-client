use futures::channel::oneshot;
use libp2p::multiaddr::Protocol;
use libp2p::PeerId;
use parking_lot::Mutex;
use std::sync::Arc;
use subspace_networking::{
    BootstrappedNetworkingParameters, Config, MemoryProviderStorage, Node,
    PieceByHashRequestHandler,
};

pub async fn configure_dsn(bootstrap_address: String) -> Node {
    let config_1 = Config::<MemoryProviderStorage> {
        listen_on: vec!["/ip4/0.0.0.0/tcp/40001".parse().unwrap()],
        allow_non_global_addresses_in_dht: false,
        networking_parameters_registry: BootstrappedNetworkingParameters::new(vec![
            bootstrap_address.parse().unwrap(),
        ])
        .boxed(),
        request_response_protocols: vec![PieceByHashRequestHandler::create(|_| async { None })],
        provider_storage: MemoryProviderStorage::new(PeerId::random()),
        ..Config::default()
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
