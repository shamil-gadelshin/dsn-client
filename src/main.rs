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
//        "/ip4/127.0.0.1/tcp/60020/p2p/12D3KooWCMSBsA3b4GyndmeehCumxbogUix5GNqALRkauTmfxtWb"
        "/ip4/127.0.0.1/tcp/50000/p2p/12D3KooWGAjyJAZNNsHu8sV6MP6mXHzNXFQbadjVBFUr5deTiom2"
//    "/ip4/127.0.0.1/tcp/50020/p2p/12D3KooWNBpUGhjg57pjGqXsEQTRRGBVveAgU5b3JTponfBYrb7V"
 //       "/dns/bootstrap-0.devnet.subspace.network/tcp/30533/p2p/12D3KooWJgLU8DmkXwBpQtHgSURFfJ4f2SyuNVBgVY96aDJsDWFK"
            .to_string();

    let protocol_prefix = "4dfbc27ccd94e335e425535e2aac790b1f016f9e1c546edd4b560a251e7382bd";

    let node = configure_dsn(bootstrap_address, protocol_prefix).await;

//    let _ = node.wait_for_connected_peers(Duration::from_secs(100)).await.unwrap();
    info!("Connected to DSN.");
 //   let piece_index: PieceIndex = 95u64.into();
    let piece_index: PieceIndex = 20u64.into();

    //   sleep(Duration::from_secs(10)).await;
    // announce_single_piece_index_hash_with_backoff(piece_index.hash(), &node.clone()).await.unwrap();
    //
    let providers = get_providers(node.clone(), piece_index).await;
    info!("Providers: {:?}", providers);

 //   let key = piece_index.hash().to_multihash();
 //    let key = PeerId::random();
 //    let peers = get_closest_peers(node.clone(), key.as_ref().clone()).await;
 //    info!("Closest peers: {:?}", peers);

    // sleep(Duration::from_secs(10)).await;

    // let piece = get_piece_from_storage(node.clone(), piece_index).await;
    // info!("piece: {:?}", piece.map(|i| i.len()).unwrap_or_default());

    // let peer_id = PeerId::from_str("12D3KooWCMSBsA3b4GyndmeehCumxbogUix5GNqALRkauTmfxtWb").unwrap();
    // let piece = get_piece_by_hash(node.clone(), peer_id, piece_index).await;
    // info!("piece: {:?}", piece.map(|i| i.len()).unwrap_or_default());
}


