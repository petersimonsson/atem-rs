use anyhow::Result;
use atem_rs::{Client, Message};
use clap::Parser;

#[derive(Debug, Parser)]
struct Args {
    address: Box<str>,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let args = Args::parse();

    let (_client, mut eventloop) = Client::connect(&args.address).await?;

    loop {
        match eventloop.poll().await {
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
