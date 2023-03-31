use futures::StreamExt;
use subspace_core_primitives::{Piece, PieceIndex, PieceIndexHash};
use subspace_networking::utils::multihash::ToMultihash;
use subspace_networking::{Node, PieceByHashRequest, PieceByHashResponse};
use tracing::{debug, info, trace, warn};

// Get from piece cache (L2) or archival storage (L1)
pub async fn get_piece_from_storage(dsn_node: Node, piece_index: PieceIndex) -> Option<Piece> {
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