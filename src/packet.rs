use bytes::{Buf, BufMut, Bytes, BytesMut};

const HEADER_SIZE: u16 = 0x0c;

pub const PACKET_FLAG_ACK_REQUEST: u8 = 0x01;
pub const PACKET_FLAG_HELLO: u8 = 0x02;
#[allow(dead_code)]
pub const PACKET_FLAG_RESEND: u8 = 0x04;
pub const PACKET_FLAG_ACK: u8 = 0x10;

#[derive(Debug, PartialEq)]
pub struct Packet {
    flags: u8,
    uid: u16,
    ack_id: u16,
    id: u16,

    payload: Option<Bytes>,
}

impl Packet {
    pub fn new(flags: u8, uid: u16, ack_id: u16, id: u16, payload: Option<Bytes>) -> Self {
        Packet {
            flags,
            uid,
            ack_id,
            id,
            payload,
        }
    }

    pub fn serialize(&self) -> Bytes {
        let mut bytes = BytesMut::new();

        let payload_size = if let Some(p) = &self.payload {
            p.len() as u16
        } else {
            0
        };
        let size_flags = ((self.flags as u16) << 11) | (payload_size + HEADER_SIZE);

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

    pub fn deserialize(mut packet: Bytes) -> Self {
        let flag_size = packet.get_u16();
        let flags = ((flag_size & 0xf800) >> 11) as u8;
        let uid = packet.get_u16();
        let ack_id = packet.get_u16();
        packet.get_u32();
        let id = packet.get_u16();

        let payload = if packet.remaining() > 0 {
            Some(packet)
        } else {
            None
        };

        Packet {
            flags,
            uid,
            ack_id,
            id,
            payload,
        }
    }

    pub fn flags(&self) -> u8 {
        self.flags
    }

    pub fn id(&self) -> u16 {
        self.id
    }

    pub fn uid(&self) -> u16 {
        self.uid
    }

    pub fn payload(&self) -> Option<Bytes> {
        self.payload.clone()
    }

    pub fn new_hello_packet() -> Self {
        let mut hello_data = BytesMut::new();
        hello_data.extend_from_slice(&[0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);

        Packet::new(
            PACKET_FLAG_HELLO,
            0x1337,
            0x0000,
            0x0000,
            Some(hello_data.freeze()),
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
            PACKET_FLAG_HELLO,
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

        let packet = Packet::deserialize(buf.freeze());

        let mut hello_data = BytesMut::new();
        hello_data.extend_from_slice(&[0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
        let expected = Packet::new(
            PACKET_FLAG_HELLO,
            0x5706,
            0x0000,
            0x0000,
            Some(hello_data.freeze()),
        );

        assert_eq!(packet, expected);
    }
}
