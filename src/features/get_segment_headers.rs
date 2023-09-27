#![allow(dead_code)]

use libp2p::PeerId;
use subspace_core_primitives::SegmentHeader;
use subspace_networking::{Node, SegmentHeaderRequest, SegmentHeaderResponse};
use tracing::{info, warn};

pub async fn get_segment_headers(
    dsn_node: Node,
    peer_id: PeerId,
    last: bool,
) -> Vec<SegmentHeader> {
    let request = if last {
        SegmentHeaderRequest::LastSegmentHeaders {
            segment_header_number: 10,
        }
    } else {
        SegmentHeaderRequest::SegmentIndexes {
            segment_indexes: vec![4.into()],
        }
    };

    let request_result = dsn_node.send_generic_request(peer_id, request).await;

    match request_result {
        Ok(SegmentHeaderResponse { segment_headers }) => {
            info!(%peer_id, "Segment headers request succeeded.");
            return segment_headers;
        }
        Err(error) => {
            warn!(%peer_id, ?error, "Segment headers request failed.");
        }
    }

    return Vec::new();
}
