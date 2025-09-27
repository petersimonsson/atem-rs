use bitflags::bitflags;
use bytes::{Buf, BufMut, Bytes, BytesMut};

const HEADER_SIZE: u16 = 0x0c;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct PacketFlag: u8 {
        const ACK_REQUEST = 0x01;
        const HELLO = 0x02;
        #[allow(dead_code)]
        const RESEND = 0x04;
        const ACK = 0x10;
    }
}

#[derive(Debug, PartialEq)]
pub struct Packet {
    flags: Option<PacketFlag>,
    uid: u16,
    ack_id: u16,
    id: u16,

    payload: Option<Bytes>,
}

impl Packet {
    pub fn new(
        flags: Option<PacketFlag>,
        uid: u16,
        ack_id: u16,
        id: u16,
        payload: Option<Bytes>,
    ) -> Self {
        Packet {
            flags,
            uid,
            ack_id,
            id,
            payload,
        }
    }

    pub fn new_ack(uid: u16, ack_id: u16, id: u16) -> Self {
        Packet::new(Some(PacketFlag::ACK), uid, ack_id, id, None)
    }

    pub fn serialize(&self) -> Bytes {
        let mut bytes = BytesMut::new();

        let payload_size = if let Some(p) = &self.payload {
            p.len() as u16
        } else {
            0
        };
        let size_flags = if let Some(flags) = self.flags {
            ((flags.bits() as u16) << 11) | (payload_size + HEADER_SIZE)
        } else {
            payload_size + HEADER_SIZE
        };

        bytes.put_u16(size_flags);
        bytes.put_u16(self.uid);
        bytes.put_u16(self.ack_id);
        bytes.put_u32(0x00);
        bytes.put_u16(self.id);

        if let Some(payload) = &self.payload {
            bytes.extend_from_slice(payload);
        }

        bytes.freeze()
    }

    pub fn deserialize(packet: &mut Bytes) -> Self {
        let flag_size = packet.get_u16();
        let flags = ((flag_size & 0xf800) >> 11) as u8;
        let size = flag_size & 0x07ff;
        let uid = packet.get_u16();
        let ack_id = packet.get_u16();
        packet.get_u32();
        let id = packet.get_u16();

        let payload_size = size - HEADER_SIZE;
        let payload = if payload_size > 0 {
            Some(packet.split_to(payload_size as usize))
        } else {
            None
        };

        Packet {
            flags: PacketFlag::from_bits(flags),
            uid,
            ack_id,
            id,
            payload,
        }
    }

    pub fn id(&self) -> u16 {
        self.id
    }

    pub fn uid(&self) -> u16 {
        self.uid
    }

    pub fn ack_request(&self) -> bool {
        if let Some(flags) = self.flags {
            !(flags & PacketFlag::ACK_REQUEST).is_empty()
        } else {
            false
        }
    }

    pub fn is_hello(&self) -> bool {
        if let Some(flags) = self.flags {
            !(flags & PacketFlag::HELLO).is_empty()
        } else {
            false
        }
    }

    pub fn payload(&self) -> Option<Bytes> {
        self.payload.clone()
    }

    pub fn new_hello_packet() -> Self {
        let hello_data = Bytes::from(vec![0x01u8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);

        Packet::new(
            Some(PacketFlag::HELLO),
            0x1337,
            0x0000,
            0x0000,
            Some(hello_data),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn packet_serialize_ok() {
        let mut hello_data = BytesMut::new();
        hello_data.extend_from_slice(&[0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
        let packet = Packet::new(
            Some(PacketFlag::HELLO),
            0x5706,
            0x0000,
            0x0000,
            Some(hello_data.freeze()),
        );
        let expected: [u8; HEADER_SIZE as usize + 0x08] = [
            0x10, 0x14, 0x57, 0x06, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];

        assert_eq!(packet.serialize().to_vec(), expected);
    }

    #[test]
    fn packet_deserialize_ok() {
        let data: [u8; HEADER_SIZE as usize + 0x08] = [
            0x10, 0x14, 0x57, 0x06, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];
        let mut buf = BytesMut::new();
        buf.extend_from_slice(&data);

        let mut packets = buf.freeze();
        let packet = Packet::deserialize(&mut packets);

        let mut hello_data = BytesMut::new();
        hello_data.extend_from_slice(&[0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
        let expected = Packet::new(
            Some(PacketFlag::HELLO),
            0x5706,
            0x0000,
            0x0000,
            Some(hello_data.freeze()),
        );

        assert_eq!(packet, expected);
    }
}
