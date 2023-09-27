use futures::StreamExt;
use libp2p::PeerId;
use subspace_networking::{Multihash, Node};
use tracing::{info, warn};

#[allow(dead_code)]
pub async fn get_closest_peers(dsn_node: Node, key: Multihash) -> Vec<PeerId> {
    let get_peers_result = dsn_node.get_closest_peers(key).await;
    let mut peers = Vec::new();
    match get_peers_result {
        Ok(mut get_peers_stream) => {
            while let Some(peer_id) = get_peers_stream.next().await {
                info!(?key, %peer_id, "get_closest_peers returned an item");

                peers.push(peer_id);
            }
        }
        Err(err) => {
            warn!(?key, ?err, "get_closest_peers returned an error");
        }
    }

    peers
}
