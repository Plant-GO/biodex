use biodex::types::args::Args;
use clap::Parser;
use dotenv::dotenv;
use solana_sdk::signer::{
    keypair::{self, Keypair},
    Signer,
};

fn main() {
    let args = Args::parse();
    let _ = dotenv();

    match log4rs::init_file(&args.log_config, Default::default()) {
        Ok(()) => log::info!("Logger successfully initialized for biodex!"),
        Err(e) => log::error!("Logger couldn't be initialized: {e}"),
    }
    log::info!("Biodex initialized!");

    if let Err(e) = dotenv::from_path(&args.dotenv) {
        log::error!("Error with .env file: {e}");
    }

    log::info!("Hello world!");

    let keypair = Keypair::new();
    println!("Public Key: {}", keypair.pubkey());
    println!("Secret Key: {:?}", keypair.to_bytes());
}
