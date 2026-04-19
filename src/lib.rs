use std::net::{IpAddr, SocketAddr};

use tokio::{net::UdpSocket, sync::mpsc};
use zerocopy::{FromBytes, IntoBytes};

use crate::protocol::{
    CommandHeader, EventResult, HelloData, Packet, PacketFlag, PacketHeader, parse_command,
};

pub mod protocol;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("Event channel closed")]
    EventChannelClosed,
}

pub type Result<T> = std::result::Result<T, Error>;

pub struct Client {}

impl Client {
    pub async fn new(address: IpAddr) -> Result<(Self, EventLoop)> {
        let (event_loop, m_event_tx) = EventLoop::new(100);
        start_event_loop(SocketAddr::new(address, 9910), m_event_tx).await?;

        Ok((Self {}, event_loop))
    }
}

pub struct EventLoop {
    event_rx: mpsc::Receiver<EventResult>,
}

impl EventLoop {
    pub fn new(queue_size: usize) -> (Self, mpsc::Sender<EventResult>) {
        let (event_tx, event_rx) = mpsc::channel(queue_size);
        (Self { event_rx }, event_tx)
    }

    pub async fn next_event(&mut self) -> Result<EventResult> {
        self.event_rx.recv().await.ok_or(Error::EventChannelClosed)
    }
}

async fn start_event_loop(address: SocketAddr, event_tx: mpsc::Sender<EventResult>) -> Result<()> {
    let mut state = EventLoopState::Unconnected;
    let socket = UdpSocket::bind("0.0.0.0:0").await?;

    tokio::spawn(async move {
        let mut session_id = 0x0001;
        let mut sequence_id = 0x0000;

        loop {
            let mut buf: [u8; 1500] = [0; 1500];
            match state {
                EventLoopState::Unconnected => {
                    match socket
                        .send_to(
                            Packet::new(
                                PacketFlag::HELLO,
                                session_id,
                                0x0000,
                                0x0000,
                                HelloData::new(0x01),
                            )
                            .as_bytes(),
                            address,
                        )
                        .await
                    {
                        Ok(size) => {
                            println!("Sent {} bytes", size);
                            state = EventLoopState::WaitingForHello;
                        }
                        Err(e) => {
                            println!("Error: {}", e);
                            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                        }
                    }
                }
                EventLoopState::WaitingForHello => match socket.recv(&mut buf).await {
                    Ok(size) => {
                        let hello_packet =
                            Packet::<HelloData>::ref_from_bytes(&buf[..size]).unwrap();

                        if hello_packet.header().session_id() != session_id {
                            session_id = hello_packet.header().session_id();
                        }

                        match socket
                            .send_to(
                                PacketHeader::ack(
                                    hello_packet.header().session_id(),
                                    0x0000,
                                    0x0000,
                                )
                                .as_bytes(),
                                address,
                            )
                            .await
                        {
                            Ok(_size) => {
                                state = EventLoopState::SettingUpSession;
                            }
                            Err(e) => {
                                println!("Error: {}", e);
                                state = EventLoopState::Unconnected;
                            }
                        }
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                        state = EventLoopState::Unconnected;
                    }
                },
                EventLoopState::SettingUpSession => match socket.recv(&mut buf).await {
                    Ok(size) => {
                        let (header, mut remaining_packet) =
                            PacketHeader::ref_from_prefix(&buf[..size]).unwrap();

                        if header.session_id() != session_id {
                            session_id = header.session_id();
                        }

                        if header.flags() == Some(PacketFlag::ACK_REQUEST | PacketFlag::ACK) {
                            match socket
                                .send_to(
                                    PacketHeader::ack(
                                        session_id,
                                        header.sequence_id(),
                                        sequence_id,
                                    )
                                    .as_bytes(),
                                    address,
                                )
                                .await
                            {
                                Ok(_size) => {
                                    println!("Sent ACK for {:02X}", header.sequence_id());
                                    state = EventLoopState::Connected;
                                }
                                Err(e) => {
                                    println!("Error: {}", e);
                                    state = EventLoopState::Unconnected;
                                }
                            }
                        }

                        while !remaining_packet.is_empty() {
                            let (command_header, remaining) =
                                match CommandHeader::ref_from_prefix(remaining_packet) {
                                    Ok(res) => res,
                                    Err(e) => {
                                        eprintln!("Error: {}", e);
                                        continue;
                                    }
                                };
                            let (data, remaining) =
                                remaining.split_at(command_header.data_size() as usize);
                            remaining_packet = remaining;
                            let result = parse_command(command_header, data);

                            if event_tx.send(result).await.is_err() {
                                eprintln!("Event channel closed");
                                return;
                            }
                        }
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                        state = EventLoopState::Unconnected;
                    }
                },
                EventLoopState::Connected => match socket.recv(&mut buf).await {
                    Ok(size) => {
                        let (header, mut remaining_packet) =
                            PacketHeader::ref_from_prefix(&buf[..size]).unwrap();

                        if header.session_id() != session_id {
                            session_id = header.session_id();
                        }

                        if let Some(flags) = header.flags()
                            && flags.contains(PacketFlag::ACK_REQUEST)
                        {
                            sequence_id += 1;
                            match socket
                                .send_to(
                                    PacketHeader::ack(
                                        session_id,
                                        header.sequence_id(),
                                        sequence_id,
                                    )
                                    .as_bytes(),
                                    address,
                                )
                                .await
                            {
                                Ok(_size) => {
                                    println!("Sent ACK for {:02X}", header.sequence_id());
                                }
                                Err(e) => {
                                    println!("Error: {}", e);
                                    state = EventLoopState::Unconnected;
                                }
                            }
                        }

                        while !remaining_packet.is_empty() {
                            let (command_header, remaining) =
                                match CommandHeader::ref_from_prefix(remaining_packet) {
                                    Ok(res) => res,
                                    Err(e) => {
                                        eprintln!("Error: {}", e);
                                        continue;
                                    }
                                };
                            let (data, remaining) =
                                remaining.split_at(command_header.data_size() as usize);
                            remaining_packet = remaining;
                            let result = parse_command(command_header, data);

                            if event_tx.send(result).await.is_err() {
                                eprintln!("Event channel closed");
                                return;
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                        state = EventLoopState::Unconnected;
                    }
                },
            }
        }
    });

    Ok(())
}

pub enum EventLoopState {
    Unconnected,
    WaitingForHello,
    SettingUpSession,
    Connected,
}
