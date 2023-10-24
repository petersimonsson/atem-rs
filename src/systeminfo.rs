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
pub enum Input {
    SDI = 1,
    HDMI = 2,
    Composite = 3,
    Component = 4,
    SVideo = 5,
}

impl Input {
    fn from_u16(value: u16) -> Option<Self> {
        match value {
            1 => Some(Input::SDI),
            2 => Some(Input::HDMI),
            3 => Some(Input::Composite),
            4 => Some(Input::Component),
            5 => Some(Input::SVideo),
            _ => None,
        }
    }
}

impl fmt::Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let output = match self {
            Input::SDI => "SDI",
            Input::HDMI => "HDMI",
            Input::Composite => "Composite",
            Input::Component => "Component",
            Input::SVideo => "S-Video",
        };

        write!(f, "{}", output)
    }
}

#[derive(Debug)]
pub enum SourceType {
    External = 0,
    Black = 1,
    ColorBars = 2,
    ColorGenerator = 3,
    MediaPlayerFill = 4,
    MediaPlayerKey = 5,
    SuperSource = 6,
    MEOutput = 128,
    Auxiliary = 129,
    Mask = 130,
}

impl SourceType {
    fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(SourceType::External),
            1 => Some(SourceType::Black),
            2 => Some(SourceType::ColorBars),
            3 => Some(SourceType::ColorGenerator),
            4 => Some(SourceType::MediaPlayerFill),
            5 => Some(SourceType::MediaPlayerKey),
            6 => Some(SourceType::SuperSource),
            128 => Some(SourceType::MEOutput),
            129 => Some(SourceType::Auxiliary),
            130 => Some(SourceType::Mask),
            _ => None,
        }
    }
}

impl fmt::Display for SourceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let output = match self {
            SourceType::External => "External",
            SourceType::Black => "Black",
            SourceType::ColorBars => "Color Bars",
            SourceType::ColorGenerator => "Color Generator",
            SourceType::MediaPlayerFill => "Media Player Fill",
            SourceType::MediaPlayerKey => "Media Player Key",
            SourceType::SuperSource => "SuperSource",
            SourceType::MEOutput => "ME Output",
            SourceType::Auxiliary => "Auxiliary",
            SourceType::Mask => "Mask",
        };

        write!(f, "{}", output)
    }
}

#[derive(Debug)]
pub struct Source {
    id: u16,
    name: Option<String>,
    short_name: Option<String>,
    available_inputs: u16,
    active_input: Option<Input>,
    source_type: SourceType,
    available_functions: u8,
    available_on_me: u8,
}

impl Source {
    pub fn parse(data: &mut Bytes) -> Result<Self, parser::Error> {
        let id = data.get_u16();
        let name = parse_str(&mut data.split_to(20))?;
        let short_name = parse_str(&mut data.split_to(4))?;
        data.get_u16(); // Skip 2 bytes
        let available_inputs = data.get_u16();
        let active_input = Input::from_u16(data.get_u16());
        let source_type = SourceType::from_u8(data.get_u8()).unwrap();
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

    fn available_inputs_str(&self) -> String {
        let mut res = String::new();

        if self.available_inputs & 0x0001 > 0 {
            res += "SDI";
        }

        if self.available_inputs & 0x0002 > 0 {
            if !res.is_empty() {
                res += ", ";
            }

            res += "HDMI";
        }

        if self.available_inputs & 0x0004 > 0 {
            if !res.is_empty() {
                res += ", ";
            }

            res += "Composite";
        }

        if self.available_inputs & 0x0008 > 0 {
            if !res.is_empty() {
                res += ", ";
            }

            res += "Component";
        }

        if self.available_inputs & 0x0010 > 0 {
            if !res.is_empty() {
                res += ", ";
            }

            res += "S-Video";
        }

        res
    }
}

impl fmt::Display for Source {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let input_str = if let Some(input) = &self.active_input {
            format!("[{}] -> {}", self.available_inputs_str(), input)
        } else {
            "".to_string()
        };

        write!(
            f,
            "Source {}: {} ({}), {}, {}, [{:08b}], [{:08b}]",
            self.id,
            self.name.as_deref().unwrap_or(""),
            self.short_name.as_deref().unwrap_or(""),
            input_str,
            self.source_type,
            self.available_functions,
            self.available_on_me,
        )
    }
}
