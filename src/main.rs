#![allow(dead_code, unused_imports, unreachable_code)]

use futures::channel::oneshot;
use futures::StreamExt;
use libp2p::multiaddr::Protocol;
use libp2p::PeerId;
use parking_lot::Mutex;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use subspace_core_primitives::crypto::kzg::{Kzg};
use subspace_core_primitives::{Piece, PieceIndex, PieceIndexHash, ArchivedHistorySegment, SegmentIndex, SegmentHeader};
use subspace_farmer::utils::node_piece_getter::NodePieceGetter;
use subspace_networking::utils::multihash::{MultihashCode, ToMultihash};
use subspace_networking::utils::piece_provider::{NoPieceValidator, PieceProvider};
use subspace_networking::{
    BootstrappedNetworkingParameters, Config, MemoryProviderStorage, Node, PieceByHashRequest,
    PieceByHashRequestHandler, PieceByHashResponse,
    SegmentHeaderBySegmentIndexesRequestHandler, SegmentHeaderRequest, SegmentHeaderResponse
};
use tokio::time::sleep;
use tracing::{debug, info, trace, warn};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // let bootstrap_address =
    //     "/ip4/127.0.0.1/tcp/50000/p2p/12D3KooWGAjyJAZNNsHu8sV6MP6mXHzNXFQbadjVBFUr5deTiom2"
    //         .to_string();

    /*
          "/dns/bootstrap-0.gemini-3c.subspace.network/tcp/30433/p2p/12D3KooWF9LjJX8XSwh3WfyyZYT7TepkW84fV6LLidUBprXjmgZ9",
      "/dns/bootstrap-1.gemini-3c.subspace.network/tcp/30433/p2p/12D3KooWENiAuwtJiQVZBrAanqNrwppGXwhhV9YTVKqsasSG2wCv",
      "/dns/bootstrap-2.gemini-3c.subspace.network/tcp/30433/p2p/12D3KooWM4NyQbGQ395o7UTsd7pZ64bne1kin3Hhe5VZvFXa7SzX"

      */

    /*
          "/dns/bootstrap-0.gemini-3d.subspace.network/tcp/30433/p2p/12D3KooWG4rXVY4Z6rv2ZTqw1debMQVeGNUEFvDNkEXLTmKbqAg6",
      "/dns/bootstrap-1.gemini-3d.subspace.network/tcp/30433/p2p/12D3KooWN4Uu1Lb8p6skmMNFYYKksQdjQXh7Nvbq5jxXGr5gDQyF",
      "/dns/bootstrap-0.gemini-3d.subspace.network/tcp/50000/p2p/12D3KooWRNoaozU9DuRRWq7cMPYtcmGvXSjsxKwvTyv49FVhctQA",
      "/dns/bootstrap-1.gemini-3d.subspace.network/tcp/50000/p2p/12D3KooWBLiyiL1iwTCUx5jzbYZy1JziyPJsV6hvpe4YbufaQp65"
    */

   let bootstrap_address =
       "/dns/bootstrap-0.gemini-3d.subspace.network/tcp/50000/p2p/12D3KooWRNoaozU9DuRRWq7cMPYtcmGvXSjsxKwvTyv49FVhctQA"
//   "/dns/bootstrap-1.gemini-3c.subspace.network/tcp/30433/p2p/12D3KooWENiAuwtJiQVZBrAanqNrwppGXwhhV9YTVKqsasSG2wCv"
 //   "/ip4/65.108.74.115/tcp/30433/p2p/12D3KooWDnkL3J3rFgCVJmv7QbQtUGMrEVPDVCi8SUiv9Ggr9PDD"
 //      "/dns/bootstrap-2.gemini-3c.subspace.network/tcp/30433/p2p/12D3KooWM4NyQbGQ395o7UTsd7pZ64bne1kin3Hhe5VZvFXa7SzX"
 //      "/ip4/65.108.232.57/tcp/30433/p2p/12D3KooWM4NyQbGQ395o7UTsd7pZ64bne1kin3Hhe5VZvFXa7SzX"
 //   "/dns/bootstrap-0.devnet.subspace.network/tcp/30433/p2p/12D3KooWB53PGjUKJ7gqbtfbu1p1twuUhax48fpCGBYowFjX5vm7"
 //   "/ip4/127.0.0.1/tcp/30433/p2p/12D3KooWB53PGjUKJ7gqbtfbu1p1twuUhax48fpCGBYowFjX5vm7"
           .to_string();

    let node = configure_dsn(bootstrap_address).await;

    let _ = node.wait_for_connected_peers().await.unwrap();
    println!("Connected to DSN.");

    let piece_index: PieceIndex = 200.into();

    //  let key = PieceIndexHash::from_index(piece_index).to_multihash();

    let piece = get_piece_from_storage(node.clone(), piece_index).await;


    println!("piece: {:?}", piece.map(|i| i.len()).unwrap_or_default());

//    sleep(Duration::from_secs(2)).await;

    // let mut result = node.get_providers(key).await.unwrap();
    //
    // let mut provider_count = 0;
    // while let Some(provider) = result.next().await {
    //     println!("get_providers: {:?}", provider);
    //
    //     if provider_count > 5 {
    //         break;
    //     }
    //
    //     provider_count +=1;
    //     let segment_index = 0;
    //     let root_blocks = get_root_blocks_request(node.clone(), provider, RootBlockRequest { segment_indexes: vec![segment_index] }).await;
    // //     let root_blocks = get_root_blocks_request(node.clone(), provider, RootBlockRequest::SegmentIndexes { segment_indexes: vec![segment_index] }).await;
    //
    //     println!("Root block (get_root_blocks_request): {:?}", root_blocks);
    // }
}

async fn configure_dsn(bootstrap_address: String) -> Node {
    let config_1 = Config::<MemoryProviderStorage> {
        listen_on: vec!["/ip4/0.0.0.0/tcp/40001".parse().unwrap()],
 //       allow_non_global_addresses_in_dht: false,
        allow_non_global_addresses_in_dht: false,
        networking_parameters_registry: BootstrappedNetworkingParameters::new(vec![
            bootstrap_address.parse().unwrap(),
        ])
        .boxed(),
        request_response_protocols: vec![
            PieceByHashRequestHandler::create(|_| async { None }),
//            PieceByHashRequestHandler::create(|_, _| async { None }),
 //           RootBlockBySegmentIndexesRequestHandler::create(|_| async { None }),
 //           RootBlockBySegmentIndexesRequestHandler::create(|_, _| async { None }),
        ],
        provider_storage: MemoryProviderStorage::new(PeerId::random()),
        ..Config::default()
    };
    let (node, mut node_runner_1) = subspace_networking::create(config_1).unwrap();
    //let (node, mut node_runner_1) = subspace_networking::create(config_1).await.unwrap();

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

async fn get_root_blocks_request(
    dsn_node: Node,
    provider_id: PeerId,
    request: SegmentHeaderRequest,
// ) -> Option<Vec<Option<ArchivedHistorySegment>>> {
    ) -> Option<Vec<SegmentHeader>> {
    let request_result = dsn_node
        .send_generic_request(provider_id, request.clone())
        .await;

    match request_result {
        Ok(SegmentHeaderResponse { segment_headers }) => {
            info!(%provider_id, ?request,"Root block request succeeded.");
        //    return Some(root_blocks.clone());
            return Some(segment_headers.clone());
        }
        Err(error) => {
            warn!(%provider_id, ?request, ?error, "Root block  request failed.");
        }
    }

    None
}

// Get from piece cache (L2) or archival storage (L1)
async fn get_piece_from_storage(dsn_node: Node, piece_index: PieceIndex) -> Option<Piece> {
    let key = PieceIndexHash::from_index(piece_index).to_multihash();
    let hash = PieceIndexHash::from_index(piece_index);

    let get_providers_result = dsn_node.get_providers(key).await;

    match get_providers_result {
        Ok(mut get_providers_stream) => {
            while let Some(provider_id) = get_providers_stream.next().await {
                info!(%piece_index, %provider_id, "get_providers returned an item");

                let request_result = dsn_node
                    .send_generic_request(
                        provider_id,
                        PieceByHashRequest {
                            piece_index_hash: hash,
                        },
                    )
                    .await;

                match request_result {
                    Ok(PieceByHashResponse { piece: Some(piece) }) => {
                        trace!(%provider_id, %piece_index, ?key, "Piece request succeeded.");
                        return Some(piece);
                    }
                    Ok(PieceByHashResponse { piece: None }) => {
                        debug!(%provider_id, %piece_index, ?key, "Piece request returned empty piece.");
                    }
                    Err(error) => {
                        warn!(%provider_id, %piece_index, ?key, ?error, "Piece request failed.");
                    }
                }
            }
        }
        Err(err) => {
            warn!(%piece_index,?key, ?err, "get_providers returned an error");
        }
    }

    None
}
