use serde::{Deserialize, Serialize};
use solana_pubkey::Pubkey;

#[derive(Debug, Serialize, Deserialize)]
pub struct Traded {
    pub whirlpool: Pubkey,
    pub a_to_b: bool,
    pub pre_sqrt_price: u128,
    pub post_sqrt_price: u128,
    pub input_amount: u64,
    pub output_amount: u64,
    pub input_transfer_fee: u64,
    pub output_transfer_fee: u64,
    pub lp_fee: u64,
    pub protocol_fee: u64,
}

pub fn decode_trade_log(log: &str) {
    let bytes = base64::decode_config(log, base64::STANDARD).unwrap();
    let log: Traded = bincode::deserialize(&bytes).unwrap();
    println!("{:?}", log);
}
