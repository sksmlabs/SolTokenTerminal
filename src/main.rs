mod helpers;

use anyhow::{anyhow, Result, Error};
use helpers::{keypair_gen, load_local_keypair, load_keypair_from_file, airdrop_to, transfer_to};
use solana_client::rpc_client::RpcClient;
use solana_sdk::signature::{Signer};
use std::env;


// command: cargo run -- src/config/ajay_keypair.json src/config/bjay_keypair.json

#[tokio::main]
async fn main() -> Result<()> {
    println!("\n======== Creating connection to local Solana RPC ========");

    let rpc_url = "http://127.0.0.1:8899";
    let rpc_client = RpcClient::new(rpc_url.to_string());
    println!("\nConnected to Solana RPC at localhost:8899");

    let local = load_local_keypair()?;

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
        println!(
            "Generated Keypairs for Ajay and Bjay: {:?} and {:?}",
            ajay.pubkey(),
            bjay.pubkey()
        );
        (ajay, bjay)
    };

    println!("\n======== Final Keypair Summary ========");
    println!("Ajay: {}", ajay.pubkey());
    println!("Bjay: {}", bjay.pubkey());

    println!("\n======== Balances Summary ========");
   
    let ajay_sol: f64 = airdrop_to(&rpc_client, &ajay.pubkey(), 1.0)?;
    let bjay_sol: f64 = airdrop_to(&rpc_client, &bjay.pubkey(), 1.0)?;

    println!("Balances -> ajay: {ajay_sol} SOL, bjay: {bjay_sol} SOL");

    // transfer_to(&rpc_client, &local, &ajay, &bjay.pubkey(), 1.0);

    // Confidential txns

    Ok(())
}
