#![allow(unused_imports)]

use std::str::FromStr;
use std::time::Duration;
use libp2p::PeerId;
use crate::features::configuration::configure_dsn;
use subspace_core_primitives::{PieceIndex, ArchivedHistorySegment};
use subspace_networking::utils::multihash::ToMultihash;
use tokio::time::sleep;
use tracing::info;
use crate::features::get_closest_peers::get_closest_peers;
use crate::features::get_piece::{get_piece_by_hash, get_piece_from_storage, get_providers};
use crate::features::node_client::get_app_info;

mod features;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    info!("DSN client started.");

 //   get_app_info().await;

    let bootstrap_address =
//"/dns/bootstrap-0.devnet.subspace.network/tcp/50000/p2p/12D3KooWJgLU8DmkXwBpQtHgSURFfJ4f2SyuNVBgVY96aDJsDWFK"
        "/ip4/127.0.0.1/tcp/50000/p2p/12D3KooWGAjyJAZNNsHu8sV6MP6mXHzNXFQbadjVBFUr5deTiom2"
            .to_string();

    let protocol_prefix = "7d405a90eb1b02b8516263c5979fa51b02c6c3d8f5df634ca6dd8619fca45479";

    let node = configure_dsn(bootstrap_address, protocol_prefix).await;

    let _ = node.wait_for_connected_peers(Duration::from_secs(100)).await.unwrap();
    info!("Connected to DSN.");
 //   let piece_index: PieceIndex = 95u64.into();
    let piece_index: PieceIndex = 30u64.into();

    //   sleep(Duration::from_secs(10)).await;
    // announce_single_piece_index_hash_with_backoff(piece_index.hash(), &node.clone()).await.unwrap();
    //
    // let providers = get_providers(node.clone(), piece_index).await;
    // info!("Providers: {:?}", providers);
    //
    let peers = get_closest_peers(node.clone(), piece_index.hash().to_multihash()).await;
    info!("Closest peers: {:?}", peers);

    sleep(Duration::from_secs(10)).await;

    // let piece = get_piece_from_storage(node.clone(), piece_index).await;
    // info!("piece: {:?}", piece.map(|i| i.len()).unwrap_or_default());

    // let peer_id = PeerId::from_str("12D3KooWHBSPDeUrNoyduuv6eouvVy6vBzKsJwsHPUstpY65fYrW").unwrap();
    // let piece = get_piece_by_hash(node.clone(), peer_id, piece_index).await;
    // info!("piece: {:?}", piece.map(|i| i.len()).unwrap_or_default());
}


