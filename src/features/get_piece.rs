#![allow(dead_code)]
use futures::StreamExt;
use libp2p::PeerId;
use subspace_core_primitives::{Piece, PieceIndex};
use subspace_networking::utils::multihash::ToMultihash;
use subspace_networking::{Node,};
//use subspace_networking::{Node, PieceByHashRequest, PieceByHashResponse};
use tracing::{debug, info, trace, warn};


// let piece = get_piece_from_storage(node.clone(), piece_index).await;
// println!("piece: {:?}", piece.map(|i| i.len()).unwrap_or_default());

// Get from piece cache (L2) or archival storage (L1)
// pub async fn get_piece_from_storage(dsn_node: Node, piece_index: PieceIndex) -> Option<Piece> {
//     let hash = piece_index.hash();
//     let key = hash.to_multihash();
//
//     let get_providers_result = dsn_node.get_providers(key).await;
//
//     match get_providers_result {
//         Ok(mut get_providers_stream) => {
//             while let Some(provider_id) = get_providers_stream.next().await {
//                 info!(%piece_index, %provider_id, "get_providers returned an item");
//
//                 let request_result = dsn_node
//                     .send_generic_request(
//                         provider_id,
//                         PieceByHashRequest {
//                             piece_index_hash: hash,
//                         },
//                     )
//                     .await;
//
//                 match request_result {
//                     Ok(PieceByHashResponse { piece: Some(piece) }) => {
//                         trace!(%provider_id, %piece_index, ?key, "Piece request succeeded.");
//                         return Some(piece);
//                     }
//                     Ok(PieceByHashResponse { piece: None }) => {
//                         debug!(%provider_id, %piece_index, ?key, "Piece request returned empty piece.");
//                     }
//                     Err(error) => {
//                         warn!(%provider_id, %piece_index, ?key, ?error, "Piece request failed.");
//                     }
//                 }
//             }
//         }
//         Err(err) => {
//             warn!(%piece_index,?key, ?err, "get_providers returned an error");
//         }
//     }
//
//     None
// }
pub async fn get_providers(dsn_node: Node, piece_index: PieceIndex) -> Vec<PeerId> {
    let key = piece_index.to_multihash();

    let get_providers_result = dsn_node.get_providers(key).await;
    let mut providers = Vec::new();
    match get_providers_result {
        Ok(mut get_providers_stream) => {
            while let Some(provider_id) = get_providers_stream.next().await {
                info!(%piece_index, %provider_id, "get_providers returned an item");

                providers.push(provider_id);
            }
        }
        Err(err) => {
            warn!(%piece_index,?key, ?err, "get_providers returned an error");
        }
    }

    providers
}

// pub async fn get_piece_by_hash(dsn_node: Node, peer_id: PeerId, piece_index: PieceIndex) -> Option<Piece> {
//     let request_result = dsn_node
//         .send_generic_request(
//             peer_id,
//             PieceByHashRequest {
//                 piece_index_hash: piece_index.hash(),
//             },
//         )
//         .await;
//
//     match request_result {
//         Ok(PieceByHashResponse { piece: Some(piece) }) => {
//             info!(%peer_id, %piece_index, "Piece request succeeded.");
//             return Some(piece);
//         }
//         Ok(PieceByHashResponse { piece: None }) => {
//             info!(%peer_id, %piece_index, "Piece request returned empty piece.");
//         }
//         Err(error) => {
//             warn!(%peer_id, %piece_index, ?error, "Piece request failed.");
//         }
//     }
//
//     return None;
// }
