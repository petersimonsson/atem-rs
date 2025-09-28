use std::fmt::Display;

use bytes::{Buf, Bytes};

pub struct KeyerOnAir {
    me: u8,
    keyer: u8,
    on_air: bool,
}

impl KeyerOnAir {
    pub fn parse(data: &mut Bytes) -> Self {
        let me = data.get_u8();
        let keyer = data.get_u8();
        let on_air = data.get_u8() == 1;

        Self { me, keyer, on_air }
    }
}

impl Display for KeyerOnAir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ME: {} Keyer: {} On air: {}",
            self.me, self.keyer, self.on_air
        )
    }
}

pub struct KeyerBaseProperties {
    me: u8,
    keyer: u8,
    keyer_type: u8,
    flying: bool,
    fill: u16,
    key: u16,
    mask: bool,
    mask_top: f32,
    mask_bottom: f32,
    mask_left: f32,
    mask_right: f32,
}

impl KeyerBaseProperties {
    pub fn parse(data: &mut Bytes) -> Self {
        let me = data.get_u8();
        let keyer = data.get_u8();
        let keyer_type = data.get_u8();
        data.get_u16(); // skip 2 bytes, unknown
        let flying = data.get_u8() > 0;
        let fill = data.get_u16();
        let key = data.get_u16();
        let mask = data.get_u8() > 0;
        data.get_u8(); // skip, unknown
        let mask_top = data.get_i16() as f32 / 1000.0;
        let mask_bottom = data.get_i16() as f32 / 1000.0;
        let mask_left = data.get_i16() as f32 / 1000.0;
        let mask_right = data.get_i16() as f32 / 1000.0;

        Self {
            me,
            keyer,
            keyer_type,
            flying,
            fill,
            key,
            mask,
            mask_top,
            mask_bottom,
            mask_left,
            mask_right,
        }
    }
}

impl Display for KeyerBaseProperties {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ME: {} Keyer: {} Type: {} Flying: {} Fill: {} Key: {} Mask: {} Mask top: {:.2} Mask bottom: {:.2} Mask left: {:.2} Mask right: {:.2}",
            self.me, self.keyer, self.keyer_type, self.flying, self.fill, self.key, self.mask, self.mask_top, self.mask_bottom, self.mask_left, self.mask_right
        )
    }
}
