use std::net::IpAddr;

use atem_rs::{Client, protocol::Event};
use clap::Parser;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let (client, mut event_loop) = Client::new(cli.address).await?;

    loop {
        let event = event_loop.next_event().await?;

        match event {
            Ok(Event::Time(time)) => {
                println!("Time: {}", time);
            }
            Ok(Event::Topology(topology)) => {
                println!("Topology: {:?}", topology);
            }
            Ok(Event::UnknownCommand(header, data)) => {
                println!("Unknown command: {:?} -> {:02X?}", header, data);
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}

#[derive(Debug, Parser)]
pub struct Cli {
    address: IpAddr,
}
