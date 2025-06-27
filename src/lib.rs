pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{signature::{Keypair, Signer}, pubkey::Pubkey};
    use bs58;
    use std::io::{self, BufRead};

    #[test]
    fn keygen() {
        let kp = Keypair::new();
        println!("You've generated a new Solana wallet: {}", kp.pubkey().to_string());
        println!("");
        println!("To save your wallet, copy and paste the following into a JSON file:");
        println!("{:?}", kp.to_bytes());
    }

    #[test]
    fn base58_to_wallet() {
        println!("Input your private key as a base58 string:");
        let stdin = io::stdin();
        let base58 = stdin.lock().lines().next().unwrap().unwrap();
        match bs58::decode(base58.trim()).into_vec() {
            Ok(wallet) => {
                println!("Your wallet file format is:");
                println!("{:?}", wallet);
            },
            Err(e) => {
                println!("Failed to decode base58 string: {}", e);
            }
        }
    }

    #[test]
    fn wallet_to_base58() {
        println!("Input your private key as a JSON byte array (e.g. [12,34,...]):");
        let stdin = io::stdin();
        let wallet = stdin
            .lock()
            .lines()
            .next()
            .unwrap()
            .unwrap()
            .trim_start_matches('[')
            .trim_end_matches(']')
            .split(',')
            .map(|s| s.trim().parse::<u8>().unwrap())
            .collect::<Vec<u8>>();
        println!("Your Base58-encoded private key is:");
        let base58 = bs58::encode(wallet).into_string();
        println!("{:?}", base58);
    }

    #[test]
    fn test_conversion_sample() {
        // Wallet file format
        let wallet_bytes = vec![34,46,55,124,141,190,24,204,134,91,70,184,161,181,44,122,15,172,63,62,153,150,99,255,20,2,89,105,77,41,89,253,130,27,195,134,14,66,75,242,7,132,234,160,203,109,195,116,251,144,44,28,56,231,114,50,131,185,168,138,61,35,98,78,53];
        let expected_base58 = "gdtKSTXYULQNx87fdD3YgXkzVeyFeqwtxHm6WdEb5a9YJRnHse7GQr7t5pbepsyvUCk7VVksUGhPt4SZ8JHVSkt";
        let base58 = bs58::encode(&wallet_bytes).into_string();
        assert_eq!(base58, expected_base58);

        // Now decode back
        let decoded = bs58::decode(base58).into_vec().unwrap();
        assert_eq!(decoded, wallet_bytes);
        println!("Conversion between wallet file format and Phantom base58 is correct.");
    }

  #[test]
fn test_airdrop() {
    use solana_client::rpc_client::RpcClient;
    use solana_sdk::signature::{Keypair, Signer, read_keypair_file};

    const RPC_URL: &str = "https://turbine-solanad-4cde.devnet.rpcpool.com/9a9da9cf-6db1-47dc-839a-55aca5c9c80a";

    // Import your keypair from file
    let keypair = read_keypair_file("dev_wallet.json").expect("Couldn't find wallet file");

    let client = RpcClient::new(RPC_URL);
    match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64) {
        Ok(sig) => {
            println!("Success! Check your TX here:");
            println!("https://explorer.solana.com/tx/{}?cluster=devnet", sig);
        }
        Err(err) => {
            println!("Airdrop failed: {}", err);
        }
    }
}
    #[test]
    fn transfer_sol() {
        // TODO: Implement SOL transfer logic
    }
}