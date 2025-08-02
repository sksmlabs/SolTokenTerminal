use anyhow::Result;
use solana_sdk::signature::{Keypair, Signer};
use solana_client::rpc_client::RpcClient;
use std::fs;
use serde_json;

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