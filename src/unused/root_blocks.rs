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
