use core::fmt;

use bytes::{Buf, Bytes};

use crate::parser::{self, parse_str};

#[derive(Debug)]
pub struct Version {
    major: u16,
    minor: u16,
}

impl Version {
    pub fn parse(data: &mut Bytes) -> Self {
        Version {
            major: data.get_u16(),
            minor: data.get_u16(),
        }
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}", self.major, self.minor)
    }
}

#[derive(Debug)]
pub struct Source {
    id: u16,
    name: Option<String>,
    short_name: Option<String>,
    available_inputs: u16,
    active_input: u16,
    source_type: u8,
    available_functions: u8,
    available_on_me: u8,
}

impl Source {
    pub fn parse(data: &mut Bytes) -> Result<Self, parser::Error> {
        let id = data.get_u16();
        let name = parse_str(&mut data.split_to(20))?;
        let short_name = parse_str(&mut data.split_to(4))?;
        data.get_u16(); // Skip 2 bytes
        let available_inputs = data.get_u16(); // Bit 0: SDI, 1: HDMI, 2: Component, 3: Composite,
                                               // 4: S-VIdeo 8: Internal
        let active_input = data.get_u16(); // 1 = SDI, 2 = HDMI, 3 = Composite, 4 = Component,
                                           // 5 = SVideo, 256 = Internal
        let source_type = data.get_u8(); // 0 = External, 1 = Black, 2 = Color Bars,
                                         // 3 = Color Generator, 4 = Media Player Fill,
                                         // 5 = Media Player Key, 6 = SuperSource,
                                         // 128 = ME Output, 129 = Auxiliary, 130 = Mask
        data.get_u8(); // Skip byte
        let available_functions = data.get_u8(); // Bit 0: Auxiliary, 1: Multiviewer, 2: SuperSource Art,
                                                 // 3: SuperSource Box, 4: Key Sources
        let available_on_me = data.get_u8(); // Bit 0: ME1 + Fill Sources, 1: ME2 + Fill Sources

        Ok(Source {
            id,
            name,
            short_name,
            available_inputs,
            active_input,
            source_type,
            available_functions,
            available_on_me,
        })
    }
}

impl fmt::Display for Source {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Source {}: {} ({}), [{:016b}] -> {}/{} [{:08b}] [{:08b}]",
            self.id,
            self.name.as_deref().unwrap_or(""),
            self.short_name.as_deref().unwrap_or(""),
            self.available_inputs,
            self.active_input,
            self.source_type,
            self.available_functions,
            self.available_on_me,
        )
    }
}
