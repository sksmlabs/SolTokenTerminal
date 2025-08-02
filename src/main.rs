mod helpers;

use anyhow::Result;
use solana_client::rpc_client::RpcClient;
use solana_sdk::signature::Signer;
use std::env;
use helpers::{keypair_gen, load_keypair_from_file};

#[tokio::main]
async fn main() -> Result<()> {
    println!("\n======== Creating connecting to local Solana RPC ========");

    let rpc_url = "http://127.0.0.1:8899";
    let rpc_client = RpcClient::new(rpc_url.to_string());
    println!("\nConnected to Solana RPC at localhost:8899");

    let args: Vec<String> = env::args().collect();
    
    if args.len() > 1 && (args[1] == "--help" || args[1] == "-h") {
        println!("Usage: {} [ajay_keypair_file] [bjay_keypair_file]", args[0]);
        println!("  If no arguments provided: generates new funded keypairs");
        println!("  If 2 arguments provided: loads keypairs from the specified files");
        return Ok(());
    }
    
    let (ajay, bjay) = if args.len() == 3 {
        // Load keypairs from CLI arguments
        println!("\n======== Loading Keypairs from CLI Arguments ========");
        let ajay = load_keypair_from_file(&args[1])?;
        let bjay = load_keypair_from_file(&args[2])?;
        println!("Loaded Ajay keypair: {}", ajay.pubkey());
        println!("Loaded Bjay keypair: {}", bjay.pubkey());
        (ajay, bjay)
    } else {
        // Generate new keypairs
        println!("\n======== Generating Funded Keypairs for Ajay and Bjay ========");
        let ajay = keypair_gen(&rpc_client)?;
        let bjay = keypair_gen(&rpc_client)?;
        println!("Generated Keypairs for Ajay and Bjay: {:?} and {:?}", ajay.pubkey(), bjay.pubkey());
        (ajay, bjay)
    };

    println!("\n======== Final Keypair Summary ========");
    println!("Ajay: {}", ajay.pubkey());
    println!("Bjay: {}", bjay.pubkey());

    Ok(())
}