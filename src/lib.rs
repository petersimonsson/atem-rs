mod packet;
mod parser;
mod source;
mod systeminfo;

use std::net::SocketAddr;

use bytes::BytesMut;
use thiserror::Error;
use tokio::{net::UdpSocket, sync::mpsc};
use tracing::{debug, info};

use crate::{
    packet::{Packet, PACKET_FLAG_ACK_REQUEST, PACKET_FLAG_HELLO},
    parser::parse_payload,
};

#[derive(Error, Debug)]
pub enum Error {
    #[error("Address parsing failed")]
    AddrParseError(#[from] std::net::AddrParseError),
    #[error("ATEM connection failed")]
    SocketError(#[from] std::io::Error),

    #[error("Parsing failed")]
    ParserError(#[from] parser::Error),
}

pub enum Message {
    Connected,
    Disconnected(Error),
    ParsingFailed(Error),
}

pub struct Connection {
    rx: mpsc::UnboundedReceiver<Message>,
}

impl Connection {
    /// Open a connection to a Blackmagic ATEM switcher at address
    pub async fn open(address: &str) -> Result<Self, Error> {
        let remote_addr: SocketAddr = format!("{}:9910", address).parse()?;
        let local_addr: SocketAddr = "0.0.0.0:0".parse()?;

        let socket = UdpSocket::bind(local_addr).await?;
        socket.connect(remote_addr).await?;

        info!("Local address: {}", socket.local_addr()?);
        info!("ATEM switcher address: {}", remote_addr);

        let (tx, rx) = mpsc::unbounded_channel();
        tokio::task::spawn(async move { run(socket, tx).await });

        Ok(Connection { rx })
    }

    pub async fn recv_message(&mut self) -> Option<Message> {
        self.rx.recv().await
    }
}

async fn send_hello_packet(socket: &UdpSocket) -> Result<(), Error> {
    let packet = Packet::new_hello_packet();
    socket.send(&packet.serialize()).await?;

    Ok(())
}

async fn run(socket: UdpSocket, tx: mpsc::UnboundedSender<Message>) {
    let mut packet_id = 0;

    if let Err(e) = send_hello_packet(&socket).await {
        let _ = tx.send(Message::Disconnected(e));
        return;
    }

    loop {
        let mut buf = BytesMut::with_capacity(1500);
        let len = match socket.recv_buf(&mut buf).await {
            Ok(len) => len,
            Err(e) => {
                let _ = tx.send(Message::Disconnected(e.into()));
                return;
            }
        };

        if len > 0 {
            let mut packets = buf.freeze();

            while packets.len() > 0 {
                let packet = Packet::deserialize(&mut packets);

                if packet.flags() & PACKET_FLAG_HELLO > 0 {
                    debug!("Recieved Hello packet");

                    if let Err(e) = send_ack(&socket, packet.uid(), 0x0, packet.id()).await {
                        let _ = tx.send(Message::Disconnected(e.into()));
                        return;
                    }
                    continue;
                } else if packet.flags() & PACKET_FLAG_ACK_REQUEST > 0 {
                    packet_id += 1;
                    if let Err(e) = send_ack(&socket, packet.uid(), packet_id, packet.id()).await {
                        let _ = tx.send(Message::Disconnected(e.into()));
                        return;
                    }
                }

                if let Some(mut payload) = packet.payload() {
                    if let Err(e) = parse_payload(&mut payload) {
                        let _ = tx.send(Message::ParsingFailed(e.into()));
                    }
                }
            }
        }
    }
}

async fn send_ack(socket: &UdpSocket, uid: u16, packet_id: u16, ack_id: u16) -> Result<(), Error> {
    let packet = Packet::new(packet::PACKET_FLAG_ACK, uid, ack_id, packet_id, None);

    debug!("Send Ack for {}", ack_id);

    socket.send(&packet.serialize()).await?;

    Ok(())
}
