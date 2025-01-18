use std::fmt::Display;

use bytes::{Buf, Bytes};

pub struct MultiViewInput {
    multiview: u8,
    window: u8,
    source: u16,
}

impl MultiViewInput {
    pub fn parse(data: &mut Bytes) -> Self {
        let multiview = data.get_u8();
        let window = data.get_u8();
        let source = data.get_u16();

        MultiViewInput {
            multiview,
            window,
            source,
        }
    }
}

impl Display for MultiViewInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Multiview: {} Window: {} Source: {}",
            self.multiview, self.window, self.source
        )
    }
}

pub struct MultiViewVU {
    multiview: u8,
    window: u8,
    enabled: bool,
}

impl MultiViewVU {
    pub fn parse(data: &mut Bytes) -> Self {
        let multiview = data.get_u8();
        let window = data.get_u8();
        let enabled = data.get_u8() == 1;

        MultiViewVU {
            multiview,
            window,
            enabled,
        }
    }
}

impl Display for MultiViewVU {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Multiview: {} Window: {} Enabled: {}",
            self.multiview, self.window, self.enabled
        )
    }
}

pub struct MultiViewSafeArea {
    multiview: u8,
    window: u8,
    enabled: bool,
}

impl MultiViewSafeArea {
    pub fn parse(data: &mut Bytes) -> Self {
        let multiview = data.get_u8();
        let window = data.get_u8();
        let enabled = data.get_u8() == 1;

        MultiViewSafeArea {
            multiview,
            window,
            enabled,
        }
    }
}

impl Display for MultiViewSafeArea {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Multiview: {} Window: {} Enabled: {}",
            self.multiview, self.window, self.enabled
        )
    }
}

pub struct MultiViewLayout {
    multiview: u8,
    layout: u8,
    flip_program: bool,
}

impl MultiViewLayout {
    pub fn parse(data: &mut Bytes) -> Self {
        let multiview = data.get_u8();
        let layout = data.get_u8();
        let flip_program = data.get_u8() == 1;

        MultiViewLayout {
            multiview,
            layout,
            flip_program,
        }
    }
}

impl Display for MultiViewLayout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Multiview: {} Layout: {} Flip program: {}",
            self.multiview, self.layout, self.flip_program
        )
    }
}
