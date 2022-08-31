use solana_client::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;
use solana_sdk::signature::Signature;
use std::error::Error;
const LAMPORTS_PER_SOL: f64 = 1000000000.0;

pub fn request_air_drop(
    rpc_client: &RpcClient,
    pubkey: &Pubkey,
    amount_sol: f64,
) -> Result<Signature, Box<dyn Error>> {
    let sig = rpc_client.request_airdrop(pubkey, (amount_sol * LAMPORTS_PER_SOL) as u64)?;
    loop {
        let confirmed = rpc_client.confirm_transaction(&sig)?;
        if confirmed {
            break;
        }
    }
    Ok(sig)
}

pub fn check_balance(rpc_client: &RpcClient, pubkey: &Pubkey) -> Result<f64, Box<dyn Error>> {
    Ok(rpc_client.get_balance(pubkey)? as f64 / LAMPORTS_PER_SOL)
}
