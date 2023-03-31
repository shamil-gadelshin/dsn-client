async fn main() {
    tracing_subscriber::fmt::init();

    let bootstrap_address =
        "/ip4/127.0.0.1/tcp/50000/p2p/12D3KooWGAjyJAZNNsHu8sV6MP6mXHzNXFQbadjVBFUr5deTiom2"
            .to_string();

    let node = configure_dsn(bootstrap_address).await;

    let _ = node.wait_for_connected_peers().await.unwrap();
    println!("Connected to DSN.");

    let piece_index: PieceIndex = 200;

    //  let key = PieceIndexHash::from_index(piece_index).to_multihash();

    //let indices = vec![79871, 30975, 16639, 101119, 57855];
    //let piece_index: PieceIndex = indices[0];

    let piece_provider = PieceProvider::<NoPieceValidator>::new(node.clone(), None);

    let piece_getter = NodePieceGetter::new(piece_provider);

    let missing_piece = recover_missing_piece(
        &piece_getter,
        Kzg::new(test_public_parameters()),
        piece_index,
    )
        .await
        .ok();

    let piece = get_piece_from_storage(node.clone(), piece_index).await;

    if piece == missing_piece {
        println!("It works");
    } else {
        println!("It doesn't work!!!!");
        println!(
            "missing_piece: {:?}",
            missing_piece.map(|i| i.len()).unwrap_or_default()
        );
        println!("piece: {:?}", piece.map(|i| i.len()).unwrap_or_default());
    }

    return;
}


async fn get_root_blocks_request(
    dsn_node: Node,
    provider_id: PeerId,
    request: RootBlockRequest,
// ) -> Option<Vec<Option<RootBlock>>> {
) -> Option<Vec<RootBlock>> {
    let request_result = dsn_node
        .send_generic_request(provider_id, request.clone())
        .await;

    match request_result {
        Ok(RootBlockResponse { root_blocks }) => {
            info!(%provider_id, ?request,"Root block request succeeded.");
            return Some(root_blocks.clone());
        }
        Err(error) => {
            warn!(%provider_id, ?request, ?error, "Root block  request failed.");
        }
    }

    None
}
