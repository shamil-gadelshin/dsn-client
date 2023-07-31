#![allow(dead_code)]
use subspace_farmer::{NodeClient, NodeRpcClient};

pub async fn get_app_info(){
    let node_client = NodeRpcClient::new("ws://127.0.0.1:9944").await.unwrap();

    let info = node_client.farmer_app_info().await.unwrap();

    println!("{:?}", info);
}
