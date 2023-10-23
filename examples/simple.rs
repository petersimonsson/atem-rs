use anyhow::Result;
use atem_rs::Connection;
use clap::Parser;

#[derive(Debug, Parser)]
struct Args {
    address: Box<str>,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let args = Args::parse();

    let atem = Connection::open(&args.address).await?;

    atem.process_packets().await?;

    Ok(())
}
