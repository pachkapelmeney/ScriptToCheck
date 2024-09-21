// use anchor_lang::{prelude::*};
use solana_client::rpc_client::RpcClient;
use solana_sdk::signature::{Keypair, Signer};
use solana_program::pubkey::Pubkey;
use borsh::BorshDeserialize;
use std::str::FromStr;

#[derive(BorshDeserialize, Debug)]
struct UserInfo {
    first_name: String,
    last_name: String,
}

fn main() {
    let rpc_url = "https://api.testnet.solana.com"; 
    let client = RpcClient::new(rpc_url);

    
    let program_id = Pubkey::from_str("2rGs8642zNSqWUwoSksL2LYFQUAuM12Si75drmsEDq63").unwrap();
    let user_info_account_key = Pubkey::from_str("9ZkBNyXrgyYjJZCaGYcmF2sg3fJEE2p5ZFu8ghP2LJXU").unwrap();
    
    match client.get_account(&user_info_account_key) {
        Ok(account) => {
            
            let user_info: UserInfo = BorshDeserialize::try_from_slice(&account.data).unwrap();
            println!("First name: {}", user_info.first_name);
            println!("Last name: {}", user_info.last_name);
        }
        Err(err) => {
            eprintln!("Failed to fetch account: {}", err);
        }
    }
}
 