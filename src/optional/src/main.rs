fn optional_text(string: Option<String>) {
    println!("{}", string.unwrap_or(String::from("hello there")));
}

fn cpfp_package(
    revaultd: &Arc<RwLock<RevaultD>>,
    bitcoind: &BitcoinD,
    to_be_cpfped: Vec<ToBeCpfped>,
    target_feerate: u64,
) -> Result<(), BitcoindError>;

fn main() {
    optional_text(None);
    optional_text(Some(String::from("hello")));
}
