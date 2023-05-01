use futures::StreamExt;
use libp2p::Multiaddr;
use subspace_core_primitives::{PieceIndex, PieceIndexHash};
use subspace_networking::utils::multihash::ToMultihash;
use subspace_networking::{Node, PieceAnnouncementRequest};
use tracing::{info, warn};

pub async fn announce_piece(dsn_node: Node, piece_index: PieceIndex) {
    let key = PieceIndexHash::from_index(piece_index).to_multihash();

    let get_closest_peers_result = dsn_node.get_closest_peers(key).await;

    match get_closest_peers_result {
        Ok(mut get_closest_peers_stream) => {
            while let Some(peer_id) = get_closest_peers_stream.next().await {
                info!(%piece_index, %peer_id, "get_closest_peers returned an item");

                let request_result = dsn_node
                    .send_generic_request(
                        peer_id,
                        PieceAnnouncementRequest {
                            piece_key: key.to_bytes(),
                            addresses: vec!["/ip4/0.0.0.0/tcp/40001".parse::<Multiaddr>().unwrap().to_vec()]
                        },
                    )
                    .await;

                match request_result {
                    Ok(PieceAnnouncementResponse) => {
                        info!(%peer_id, %piece_index, ?key, "Piece announcement succeeded.");
                    }
                    Err(error) => {
                        warn!(%peer_id, %piece_index, ?key, ?error, "Piece announcement failed.");
                    }
                }
            }
        }
        Err(err) => {
            warn!(%piece_index,?key, ?err, "get_closest_peers returned an error");
        }
    }
}
