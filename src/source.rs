use bitflags::bitflags;
use bytes::{Buf, Bytes};

use std::fmt;

use crate::{command, parser::parse_str};

#[derive(Debug)]
pub enum Input {
    Sdi,
    Hdmi,
    Composite,
    Component,
    SVideo,
    Internal,
    Unknown(u16),
}

impl From<u16> for Input {
    fn from(value: u16) -> Self {
        match value {
            1 => Input::Sdi,
            2 => Input::Hdmi,
            3 => Input::Composite,
            4 => Input::Component,
            5 => Input::SVideo,
            256 => Input::Internal,
            val => Input::Unknown(val),
        }
    }
}

impl From<Input> for u16 {
    fn from(value: Input) -> Self {
        match value {
            Input::Sdi => 1,
            Input::Hdmi => 2,
            Input::Composite => 3,
            Input::Component => 4,
            Input::SVideo => 5,
            Input::Internal => 256,
            Input::Unknown(val) => val,
        }
    }
}

impl fmt::Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let output = match self {
            Input::Sdi => "SDI".to_string(),
            Input::Hdmi => "HDMI".to_string(),
            Input::Composite => "Composite".to_string(),
            Input::Component => "Component".to_string(),
            Input::SVideo => "S-Video".to_string(),
            Input::Internal => "Internal".to_string(),
            Input::Unknown(val) => format!("Unknown ({val})"),
        };

        write!(f, "{}", output)
    }
}

#[derive(Debug)]
pub enum SourceType {
    External,
    Black,
    ColorBars,
    ColorGenerator,
    MediaPlayerFill,
    MediaPlayerKey,
    SuperSource,
    MEOutput,
    Auxiliary,
    Mask,
    Status,
    Direct,
    Unknown(u8),
}

impl From<u8> for SourceType {
    fn from(value: u8) -> Self {
        match value {
            0 => SourceType::External,
            1 => SourceType::Black,
            2 => SourceType::ColorBars,
            3 => SourceType::ColorGenerator,
            4 => SourceType::MediaPlayerFill,
            5 => SourceType::MediaPlayerKey,
            6 => SourceType::SuperSource,
            7 => SourceType::Direct,
            128 => SourceType::MEOutput,
            129 => SourceType::Auxiliary,
            130 => SourceType::Mask,
            131 => SourceType::Status,
            val => SourceType::Unknown(val),
        }
    }
}

impl From<SourceType> for u8 {
    fn from(value: SourceType) -> Self {
        match value {
            SourceType::External => 0,
            SourceType::Black => 1,
            SourceType::ColorBars => 2,
            SourceType::ColorGenerator => 3,
            SourceType::MediaPlayerFill => 4,
            SourceType::MediaPlayerKey => 5,
            SourceType::SuperSource => 6,
            SourceType::Direct => 7,
            SourceType::MEOutput => 128,
            SourceType::Auxiliary => 129,
            SourceType::Mask => 130,
            SourceType::Status => 131,
            SourceType::Unknown(val) => val,
        }
    }
}

impl fmt::Display for SourceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let output = match self {
            SourceType::External => "External".to_string(),
            SourceType::Black => "Black".to_string(),
            SourceType::ColorBars => "Color Bars".to_string(),
            SourceType::ColorGenerator => "Color Generator".to_string(),
            SourceType::MediaPlayerFill => "Media Player Fill".to_string(),
            SourceType::MediaPlayerKey => "Media Player Key".to_string(),
            SourceType::SuperSource => "SuperSource".to_string(),
            SourceType::MEOutput => "ME Output".to_string(),
            SourceType::Auxiliary => "Auxiliary".to_string(),
            SourceType::Mask => "Mask".to_string(),
            SourceType::Status => "Status".to_string(),
            SourceType::Direct => "Direct".to_string(),
            SourceType::Unknown(val) => format!("Unknown ({val})"),
        };

        write!(f, "{}", output)
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct InputFlags: u16 {
        const SDI = 0x0001;
        const HDMI = 0x0002;
        const COMPOSITE = 0x0004;
        const COMPONENT = 0x0008;
        const SVIDEO = 0x0010;
        const INTERNAL = 0x0100;
    }
}

impl fmt::Display for InputFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = Vec::new();

        if self.contains(InputFlags::SDI) {
            output.push("SDI");
        }
        if self.contains(InputFlags::HDMI) {
            output.push("HDMI");
        }
        if self.contains(InputFlags::COMPOSITE) {
            output.push("Composite");
        }
        if self.contains(InputFlags::COMPONENT) {
            output.push("Component");
        }
        if self.contains(InputFlags::SVIDEO) {
            output.push("S-Video");
        }
        if self.contains(InputFlags::INTERNAL) {
            output.push("Internal");
        }

        write!(f, "{}", output.join(", "))
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct FunctionFlags: u8 {
        const AUXILIARY = 0x01;
        const MULTIVIEWER = 0x02;
        const SUPERSOURCE_ART = 0x04;
        const SUPERSOURCE_BOX = 0x08;
        const KEY_SOURCES = 0x10;
    }
}

impl fmt::Display for FunctionFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = Vec::new();

        if self.contains(FunctionFlags::AUXILIARY) {
            output.push("Auxiliary");
        }
        if self.contains(FunctionFlags::MULTIVIEWER) {
            output.push("Multiviewer");
        }
        if self.contains(FunctionFlags::SUPERSOURCE_ART) {
            output.push("SuperSource Art");
        }
        if self.contains(FunctionFlags::SUPERSOURCE_BOX) {
            output.push("SuperSource Box");
        }
        if self.contains(FunctionFlags::KEY_SOURCES) {
            output.push("Key Sources");
        }

        write!(f, "{}", output.join(", "))
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct MixEffectFlags: u8 {
        const ME1 = 0x01;
        const ME2 = 0x02;
        const ME3 = 0x04;
        const ME4 = 0x08;
        const ME5 = 0x10;
        const ME6 = 0x20;
        const ME7 = 0x40;
        const ME8 = 0x80;
    }
}

impl fmt::Display for MixEffectFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = Vec::new();
        if self.contains(MixEffectFlags::ME1) {
            output.push("ME1");
        }
        if self.contains(MixEffectFlags::ME2) {
            output.push("ME2");
        }
        if self.contains(MixEffectFlags::ME3) {
            output.push("ME3");
        }
        if self.contains(MixEffectFlags::ME4) {
            output.push("ME4");
        }
        if self.contains(MixEffectFlags::ME5) {
            output.push("ME5");
        }
        if self.contains(MixEffectFlags::ME6) {
            output.push("ME6");
        }
        if self.contains(MixEffectFlags::ME7) {
            output.push("ME7");
        }
        if self.contains(MixEffectFlags::ME8) {
            output.push("ME8");
        }

        write!(f, "{}", output.join(", "))
    }
}

#[derive(Debug)]
pub struct Source {
    id: u16,
    name: Option<String>,
    short_name: Option<String>,
    available_inputs: InputFlags,
    active_input: Input,
    source_type: SourceType,
    available_functions: FunctionFlags,
    available_on_me: MixEffectFlags,
}

impl Source {
    pub fn parse(data: &mut Bytes) -> Result<Self, command::Error> {
        let id = data.get_u16();
        let name = parse_str(&mut data.split_to(20))?;
        let short_name = parse_str(&mut data.split_to(4))?;
        data.get_u16(); // Skip 2 bytes
        let available_inputs = InputFlags::from_bits(data.get_u16()).unwrap();
        let active_input = data.get_u16().into();
        let source_type = data.get_u8().into();
        data.get_u8(); // Skip byte
        let available_functions = FunctionFlags::from_bits(data.get_u8()).unwrap();
        let available_on_me = MixEffectFlags::from_bits(data.get_u8()).unwrap();

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

    pub fn id(&self) -> u16 {
        self.id
    }
}

impl fmt::Display for Source {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let input_str = format!("[{}] -> {}, ", self.available_inputs, self.active_input);

        write!(
            f,
            "Source {}: {} ({}), {}{}, [{}], [{}]",
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
