#[cfg(test)]
mod test {
    use dotenv::dotenv;
    use solana_client::nonblocking::rpc_client::RpcClient;
    use solana_pubkey::Pubkey;
    use std::env;
    use std::str::FromStr;

    #[tokio::test]
    pub async fn test_swap() {
        dotenv().ok();
        let rpc_url = env::var("RPC_URL").unwrap();
        let client = RpcClient::new(rpc_url);
        let whirlpool_address =
            Pubkey::from_str("Czfq3xZZDmsdGdUyrNLtRhGc47cXcZtLG4crryfu44zE").unwrap();
    }
}
