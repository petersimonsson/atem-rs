use anyhow::Result;
use atem_rs::{Connection, Message};
use clap::Parser;

#[derive(Debug, Parser)]
struct Args {
    address: Box<str>,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let args = Args::parse();

    let mut atem = Connection::open(&args.address).await?;

    loop {
        match atem.recv_message().await {
            Some(Message::Connected) => {}
            Some(Message::Disconnected(e)) => return Err(e.into()),
            Some(Message::ParsingFailed(e)) => println!("{}", e.to_string()),
            Some(Message::Command(c)) => {
                println!("{}", c);
            }
            None => {}
        }
    }

    //Ok(())
}
