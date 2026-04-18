use bitflags::bitflags;
use zerocopy::{BigEndian, FromBytes, Immutable, IntoBytes, KnownLayout, U16};

/// Size of the packet header in bytes. Must match the size of the C struct.
const HEADER_SIZE: u16 = 0x0c;

#[derive(Debug, FromBytes, IntoBytes, Immutable, KnownLayout)]
#[repr(C)]
pub struct PacketHeader {
    flag_size: U16<BigEndian>,
    session_id: u16,
    ack_id: u16,
    reserved: [u8; 4],
    sequence_id: u16,
}

impl PacketHeader {
    pub fn new(
        flags: PacketFlag,
        session_id: u16,
        ack_id: u16,
        sequence_id: u16,
        payload_size: u16,
    ) -> Self {
        let flag_size = (payload_size + HEADER_SIZE) | ((flags.bits() as u16) << 11);

        Self {
            flag_size: U16::new(flag_size),
            session_id,
            ack_id,
            reserved: [0; 4],
            sequence_id,
        }
    }
    pub fn packet_size(&self) -> u16 {
        self.flag_size.get() & 0x07ff
    }

    pub fn flags(&self) -> Option<PacketFlag> {
        PacketFlag::from_bits(((self.flag_size.get() & 0xf800) >> 11) as u8)
    }

    pub fn session_id(&self) -> u16 {
        self.session_id
    }

    pub fn ack_id(&self) -> u16 {
        self.ack_id
    }

    pub fn sequence_id(&self) -> u16 {
        self.sequence_id
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct PacketFlag: u8 {
        const ACK_REQUEST = 0x01;
        const HELLO = 0x02;
        const RESEND = 0x04;
        const ACK = 0x10;
    }
}

#[derive(Debug, FromBytes, IntoBytes, Immutable, KnownLayout)]
#[repr(C)]
pub struct HelloData {
    sequence_id: u8,
    reserved: [u8; 7],
}

impl HelloData {
    pub fn new(sequence_id: u8) -> Self {
        Self {
            sequence_id,
            reserved: [0; 7],
        }
    }
}

impl Default for HelloData {
    fn default() -> Self {
        Self::new(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_hello_packet() {
        let data: [u8; 0x14] = [
            0x10, 0x14, 0x57, 0x06, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];

        let (header, remaining) = PacketHeader::ref_from_prefix(data.as_ref()).unwrap();
        let hello_data = HelloData::ref_from_bytes(remaining).unwrap();

        println!(
            "{:02X?} -> {:02X?} ---- {:?}, {:02X}",
            header,
            hello_data,
            header.flags(),
            header.packet_size()
        );

        assert_eq!(header.flag_size, [0x10, 0x14]);
        assert_eq!(header.packet_size(), 0x0014);
        assert_eq!(header.flags(), Some(PacketFlag::HELLO));
        assert_eq!(header.session_id, 0x0657);
        assert_eq!(header.ack_id, 0x0000);
        assert_eq!(header.sequence_id, 0x0000);
        assert_eq!(remaining.len(), 0x08);
        assert_eq!(hello_data.sequence_id, 0x01);
    }

    #[test]
    fn serialize_hello_packet() {
        let hello_data = HelloData::default();
        let header = PacketHeader::new(
            PacketFlag::HELLO,
            0x0657,
            0x0000,
            0x0000,
            size_of_val(&hello_data) as u16,
        );
        let expected: [u8; HEADER_SIZE as usize + 0x08] = [
            0x10, 0x14, 0x57, 0x06, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];

        let mut packet: [u8; HEADER_SIZE as usize + 0x08] = [0; HEADER_SIZE as usize + 0x08];

        header.write_to_prefix(&mut packet).unwrap();
        hello_data.write_to_suffix(&mut packet).unwrap();

        assert_eq!(packet, expected);
    }

    #[test]
    fn deserialize_ack_packet() {
        let data: [u8; 0x0c] = [
            0x80, 0xc, 0x71, 0x52, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        ];

        let (header, remaining) = PacketHeader::ref_from_prefix(data.as_ref()).unwrap();

        println!("{:02X?} -> {:02X?}", header, remaining);

        assert_eq!(header.flag_size, [0x80, 0x0c]);
        assert_eq!(header.packet_size(), 0x000c);
        assert_eq!(header.flags(), Some(PacketFlag::ACK));
        assert_eq!(header.session_id, 0x5271);
        assert_eq!(header.ack_id, 0x0000);
        assert_eq!(header.sequence_id, 0x0000);
        assert_eq!(remaining.len(), 0x00);
    }

    #[test]
    fn deserialize_session_start_packet() {
        let data: [u8; 0x0c] = [0x88, 0xc, 0x80, 0x3, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0xd];

        let (header, remaining) = PacketHeader::ref_from_prefix(data.as_ref()).unwrap();

        println!("{:02X?} -> {:02X?}", header, remaining);

        assert_eq!(header.flag_size, [0x88, 0x0c]);
        assert_eq!(header.packet_size(), 0x000c);
        assert_eq!(
            header.flags(),
            Some(PacketFlag::ACK | PacketFlag::ACK_REQUEST)
        );
        assert_eq!(header.session_id, 0x0380);
        assert_eq!(header.ack_id, 0x0000);
        assert_eq!(header.sequence_id, 0x0d00);
        assert_eq!(remaining.len(), 0x00);
    }

    #[test]
    fn deserialize_ping_packet() {
        let data: [u8; 0x14] = [
            0x8, 0x14, 0x80, 0x2, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x66, 0x76, 0x0, 0x8, 0x0, 0x0,
            0x54, 0x69, 0x52, 0x71,
        ];

        let (header, remaining) = PacketHeader::ref_from_prefix(data.as_ref()).unwrap();

        println!("{:02X?} -> {:02X?}", header, remaining);

        assert_eq!(header.flag_size, [0x08, 0x14]);
        assert_eq!(header.session_id, 0x0280);
        assert_eq!(header.ack_id, 0x0000);
        assert_eq!(header.sequence_id, 0x7666);
    }

    #[test]
    fn deserialize_pong_packet() {
        let data: [u8; 0x1c] = [
            0x88, 0x1c, 0x80, 0x2, 0x66, 0x76, 0x0, 0x0, 0x0, 0x0, 0x62, 0xba, 0x0, 0x10, 0x0, 0x0,
            0x54, 0x69, 0x6d, 0x65, 0x7, 0x6, 0xd, 0xa, 0x0, 0x0, 0x0, 0x0,
        ];

        let (header, remaining) = PacketHeader::ref_from_prefix(data.as_ref()).unwrap();

        println!(
            "{:02X?} -> {:02X?} ---- {:?}, {:02X}",
            header,
            remaining,
            header.flags(),
            header.packet_size()
        );

        assert_eq!(header.flag_size, [0x88, 0x1c]);
        assert_eq!(header.packet_size(), 0x001c);
        assert_eq!(
            header.flags(),
            Some(PacketFlag::ACK | PacketFlag::ACK_REQUEST)
        );
        assert_eq!(header.session_id, 0x0280);
        assert_eq!(header.ack_id, 0x7666);
        assert_eq!(header.sequence_id, 0xba62);
    }
}
