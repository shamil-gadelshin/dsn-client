use crate::features::configuration::configure_dsn;
use subspace_core_primitives::PieceIndex;
use tokio::time::sleep;
use tracing::info;
use crate::features::announce_piece::announce_piece;
use crate::features::get_piece::get_piece_from_storage;

mod features;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    info!("DSN client started.");

    /*
      "dsnBootstrapNodes": [
      "/dns/bootstrap-0.gemini-3d.subspace.network/tcp/30433/p2p/12D3KooWG4rXVY4Z6rv2ZTqw1debMQVeGNUEFvDNkEXLTmKbqAg6",
      "/dns/bootstrap-1.gemini-3d.subspace.network/tcp/30433/p2p/12D3KooWN4Uu1Lb8p6skmMNFYYKksQdjQXh7Nvbq5jxXGr5gDQyF",
      "/dns/bootstrap-0.gemini-3d.subspace.network/tcp/50000/p2p/12D3KooWRNoaozU9DuRRWq7cMPYtcmGvXSjsxKwvTyv49FVhctQA",
      "/dns/bootstrap-1.gemini-3d.subspace.network/tcp/50000/p2p/12D3KooWBLiyiL1iwTCUx5jzbYZy1JziyPJsV6hvpe4YbufaQp65"
    ]

    */

    let bootstrap_address =
//        "/dns/bootstrap-0.gemini-3d.subspace.network/tcp/50000/p2p/12D3KooWRNoaozU9DuRRWq7cMPYtcmGvXSjsxKwvTyv49FVhctQA"
        "/ip4/127.0.0.1/tcp/50000/p2p/12D3KooWGAjyJAZNNsHu8sV6MP6mXHzNXFQbadjVBFUr5deTiom2"
 //       "/ip4/127.0.0.1/tcp/60010/p2p/12D3KooWDiDEx1tvZSYRCZxVZxfYMCw8mWRShpMJ27awXSvKh6Vs"
//        "/dns/bootstrap-0.gemini-3d.subspace.network/tcp/30433/p2p/12D3KooWG4rXVY4Z6rv2ZTqw1debMQVeGNUEFvDNkEXLTmKbqAg6"
  //      "/dns/bootstrap-1.gemini-3d.subspace.network/tcp/50000/p2p/12D3KooWBLiyiL1iwTCUx5jzbYZy1JziyPJsV6hvpe4YbufaQp65"
            .to_string();
    let protocol_prefix = "2bf60820c6dd4956739b0f1b0ce4aca0dffb3472e5021aa4d2ebc32c2e56f363";
//    let protocol_prefix = "b37acd9f4597dae33f076fc934c77ae7bb25651172bd84e7ecd03d0f8c218fb6___";
    let node = configure_dsn(bootstrap_address, protocol_prefix).await;

    let _ = node.wait_for_connected_peers().await.unwrap();
    info!("Connected to DSN.");
    let piece_index: PieceIndex = 200u64.into();
    announce_piece(node.clone(), piece_index).await;

   // let piece = get_piece_from_storage(node.clone(), piece_index).await;
   // info!("piece: {:?}", piece.map(|i| i.len()).unwrap_or_default());

}
