use crate::features::configuration::configure_dsn;
use crate::features::get_piece::get_piece_from_storage;
use subspace_core_primitives::PieceIndex;

mod features;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let bootstrap_address =
        "/ip4/127.0.0.1/tcp/50000/p2p/12D3KooWGAjyJAZNNsHu8sV6MP6mXHzNXFQbadjVBFUr5deTiom2"
            .to_string();

    let node = configure_dsn(bootstrap_address).await;

    let _ = node.wait_for_connected_peers().await.unwrap();
    println!("Connected to DSN.");

    let piece_index: PieceIndex = 200.into();

    //  let key = PieceIndexHash::from_index(piece_index).to_multihash();

    let piece = get_piece_from_storage(node.clone(), piece_index).await;

    println!("piece: {:?}", piece.map(|i| i.len()).unwrap_or_default());
}
