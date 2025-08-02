use solana_client::rpc_client::RpcClient;

#[tokio::main]
async fn main() {
    println!("\n======== Creating connecting to local Solana RPC ========");

    let rpc_url = "https://127.0.0.1:8899";
    let client = RpcClient::new(rpc_url.to_string());
    println!("\nConnected to Solana RPC at localhost:8899");
    
}