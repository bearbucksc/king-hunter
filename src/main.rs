use clap::Parser;
use std::fs;

mod wallet;
mod funder;
mod deployer;
mod buyer;
mod config;
mod logger;
mod update;
mod license; // Optional license module

#[derive(Parser)]
#[command(name = "King Hunter")]
#[command(about = "Solana Token Launching Bot", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand)]
enum Commands {
    CreateWallets { count: u32 },
    FundWallets,
    LaunchToken,
    BuyTokens { delay_ms: u64 },
    CheckUpdate,
}

fn main() {
    let cli_config = config::load();

    // Optional license check — controlled via config
    if cli_config.license.enabled {
        if !license::validate_license_key("test_key", cli_config.license.secret_key.as_bytes()) {
            eprintln!("Invalid or expired license!");
            return;
        }
    }

    let cli = Cli::parse();
    match &cli.command {
        Commands::CreateWallets { count } => wallet::create(*count),
        Commands::FundWallets => funder::fund(),
        Commands::LaunchToken => deployer::launch(),
        Commands::BuyTokens { delay_ms } => buyer::buy(*delay_ms),
        Commands::CheckUpdate => update::check_for_update("1.0.0"),
    }
}
