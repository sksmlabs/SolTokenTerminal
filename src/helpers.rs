use anyhow::Result;
use serde_json;
use solana_client::rpc_client::RpcClient;
use solana_sdk::signature::{Keypair, Signer};
use solana_sdk::native_token::LAMPORTS_PER_SOL;
use solana_sdk::commitment_config::CommitmentConfig;
use std::fs;
use std::{thread::sleep, time::Duration};

pub fn keypair_gen(client: &RpcClient) -> Result<Keypair> {
    let keypair = Keypair::new();
    let pubkey = keypair.pubkey();
    let balance = client.get_balance(&pubkey)?;

    if balance == 0 {
        let sig = client.request_airdrop(&pubkey, 1_000_000_000)?;
        client.confirm_transaction(&sig)?;

        // wait for confirmation
        loop {
            let confirmed = client.confirm_transaction(&sig)?;
            if confirmed {
                break;
            }
        }
    }

    Ok(keypair)
}

pub fn load_keypair_from_file(path: &str) -> Result<Keypair> {
    let file_content = fs::read_to_string(path)?;
    let keypair_bytes: Vec<u8> = serde_json::from_str(&file_content)?;
    let keypair = Keypair::try_from(&keypair_bytes[..])?;
    Ok(keypair)
}

pub fn lamports_to_sol(lamports: u64) -> f64 {
    lamports as f64/1_000_000_000.0
}

pub fn get_balance_or_airdrop_to(client: &RpcClient, acc_pubkey: &solana_sdk::pubkey::Pubkey, amount_sol: f64) -> Result<f64> {

    let mut lamports = client
        .get_balance_with_commitment(acc_pubkey, CommitmentConfig::finalized())?
        .value;
    let mut sol = lamports_to_sol(lamports);
    println!("Acc: {} (Balance: {} SOL)", acc_pubkey, sol);

    if sol == 0.0 {
        println!("🔄 Airdropping {:.2} SOL to {}...", amount_sol, acc_pubkey);
        let lamports_amount = (amount_sol * LAMPORTS_PER_SOL as f64) as u64;
        let blockhash = client.get_latest_blockhash()?;
        let sig = client.request_airdrop(acc_pubkey, lamports_amount)?;
        client.confirm_transaction_with_spinner(&sig, &blockhash, CommitmentConfig::finalized())?;
        println!("✅ Airdrop confirmed.");

        // fetch balance again
        lamports = client
        .get_balance_with_commitment(acc_pubkey, CommitmentConfig::finalized())?
        .value;
        sol = lamports_to_sol(lamports);
        println!("Update -> Acc: {} (Balance: {} SOL)\n", acc_pubkey, sol);
    }

    Ok(sol)
}  