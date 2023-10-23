mod packet;
mod systeminfo;

use std::net::SocketAddr;

use bytes::{Buf, Bytes, BytesMut};
use thiserror::Error;
use tokio::net::UdpSocket;
use tracing::{debug, info};

use crate::{
    packet::{Packet, PACKET_FLAG_ACK_REQUEST, PACKET_FLAG_HELLO},
    systeminfo::Version,
};

#[derive(Error, Debug)]
pub enum Error {
    #[error("Address parsing failed")]
    AddrParseError(#[from] std::net::AddrParseError),
    #[error("")]
    SocketError(#[from] std::io::Error),
    #[error("Parsing string failed")]
    Utf8Error(#[from] std::string::FromUtf8Error),
}

pub struct Connection {
    socket: UdpSocket,
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

        send_hello_packet(&socket).await?;

        Ok(Connection { socket })
    }

    pub async fn process_packets(&self) -> Result<(), Error> {
        let mut packet_id = 0;

        loop {
            let mut buf = BytesMut::with_capacity(1500);
            let len = self.socket.recv_buf(&mut buf).await?;

            if len > 0 {
                let packet = Packet::deserialize(buf.freeze());

                if packet.flags() & PACKET_FLAG_HELLO > 0 {
                    debug!("Recieved Hello packet");

                    send_ack(&self.socket, packet.uid(), 0x0, packet.id()).await?;
                    continue;
                } else if packet.flags() & PACKET_FLAG_ACK_REQUEST > 0 {
                    packet_id += 1;
                    send_ack(&self.socket, packet.uid(), packet_id, packet.id()).await?;
                }

                if let Some(mut payload) = packet.payload() {
                    parse_payload(&mut payload)?;
                }
            }
        }
    }
}

async fn send_hello_packet(socket: &UdpSocket) -> Result<(), Error> {
    let packet = Packet::new_hello_packet();
    socket.send(&packet.serialize()).await?;

    Ok(())
}

async fn send_ack(socket: &UdpSocket, uid: u16, packet_id: u16, ack_id: u16) -> Result<(), Error> {
    let packet = Packet::new(packet::PACKET_FLAG_ACK, uid, ack_id, packet_id, None);

    debug!("Send Ack for {}", ack_id);

    socket.send(&packet.serialize()).await?;

    Ok(())
}

fn parse_payload(payload: &mut Bytes) -> Result<(), Error> {
    while payload.has_remaining() {
        let size = payload.get_u16();
        payload.get_u16(); // skip two bytes, unknow function.
        let cmd = payload.split_to(4);
        let data_size = size as usize - 8;
        let mut data = payload.split_to(data_size);
        debug!("Command {:?} Size: {}", cmd, size);

        match &cmd[..] {
            b"_ver" => {
                let version = Version::parse(&mut data);
                info!("Firmware version: {}", version);
            }
            b"_pin" => {
                let product = parse_str(&mut data)?;
                info!("Product: {}", product.unwrap());
            }
            _ => {
                debug!(
                    "Unknown command: {} Data: {:02X?} [{}]",
                    String::from_utf8(cmd.to_vec())?,
                    &data[..],
                    data_size
                );
            }
        }
    }

    Ok(())
}

fn parse_str(data: &mut Bytes) -> Result<Option<String>, Error> {
    let mut data = data.splitn(2, |b| *b == b'\0');

    if let Some(str) = data.next() {
        Ok(Some(String::from_utf8(str.to_vec())?))
    } else {
        Ok(None)
    }
}
