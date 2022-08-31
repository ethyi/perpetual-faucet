use base58::FromBase58;
use perpetual_faucet::{check_balance, request_air_drop};
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::time::Instant;

const URL: &str = "https://api.devnet.solana.com";
const MAX_AIRDROP_ALLOWED: f64 = 2.0;

fn main() {
    let rpc_client = RpcClient::new(URL);

    let pubkey_string = std::env::args().nth(1).expect("no pubkey given");
    let desired_balance: f64 = std::env::args()
        .nth(2)
        .expect("no amount given")
        .parse()
        .unwrap();

    let pubkey_byte_array = pubkey_string.from_base58().expect("expected base58 string");
    // panics if not 32 bytes
    let pubkey = Pubkey::new(&pubkey_byte_array);
    println!("Receiver Public Key: {:?}", pubkey);
    let mut balance = check_balance(&rpc_client, &pubkey).expect("unable to get balance");
    println!(
        "Initial Balance: {}, Desired Balance: {}",
        balance, desired_balance
    );
    if balance >= desired_balance {
        panic!("err: Desired balance already reached");
    }

    println!("Current maximum airdrop: {} SOL", MAX_AIRDROP_ALLOWED);
    println!("Airdrops starting...");

    let mut amount_to_airdrop = desired_balance - balance;

    let start = Instant::now();
    while amount_to_airdrop > 0.0 {
        let amount = f64::min(amount_to_airdrop, MAX_AIRDROP_ALLOWED);
        let _airdrop_signature =
            request_air_drop(&rpc_client, &pubkey, amount).expect("airdrop failed");
        amount_to_airdrop -= amount;
        balance += amount;
        println!("Balance Updated: {}", balance);
    }

    println!("Time elapsed: {} seconds", start.elapsed().as_secs() as f64);

    let balance = check_balance(&rpc_client, &pubkey).expect("unable to get balance");
    println!("Receiver end balance: {:?}", balance);
}
