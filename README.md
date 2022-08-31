# Solana Devnet Perpetual Faucet
A CLI for airdropping any amount of SOL to a devnet address.

## Usage
`cargo run <DESIRED FINAL BALANCE> <PUBLIC KEY (BASE58)>`
## Implementation
It airdrops devnet SOL to your public key until the final balance is reached.

Since airdrops are capped at 2.0 SOL (8/31/22), and the devnet limits API calls, airdropping large amounts will take `10*amount/2` seconds.

Too many devnet api calls will hit the rate limit and prevent you from making additional airdrops.
For more information on rate limits see the [docs](https://docs.solana.com/cluster/rpc-endpoints).
