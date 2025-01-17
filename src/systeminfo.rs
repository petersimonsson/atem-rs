use core::fmt;
use std::collections::HashMap;

use bytes::{Buf, Bytes};

use crate::source::Source;

#[derive(Debug, Default)]
pub struct SystemInfo {
    product: Box<str>,
    version: Version,
    topology: Topology,

    sources: HashMap<u16, Source>,
}

#[allow(dead_code)]
impl SystemInfo {
    pub fn set_product(&mut self, description: &str) {
        self.product = description.into();
    }

    pub fn product(&self) -> &str {
        &self.product
    }

    pub fn set_version(&mut self, version: Version) {
        self.version = version;
    }

    pub fn version(&self) -> &Version {
        &self.version
    }

    pub fn set_topology(&mut self, topology: Topology) {
        self.topology = topology;
    }

    pub fn topology(&self) -> &Topology {
        &self.topology
    }

    pub fn set_source(&mut self, source: Source) {
        self.sources.insert(source.id(), source);
    }

    pub fn source(&self, id: u16) -> Option<&Source> {
        self.sources.get(&id)
    }
}

#[derive(Debug, Default)]
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

#[derive(Debug, Default)]
pub struct Topology {
    me_count: u8,
    source_count: u8,
    dsk_count: u8,
    aux_count: u8,
    mixminus_output_count: u8,
    mediaplayer_count: u8,
    multiviewer_count: u8,
    rs485_count: u8,
    hyperdeck_count: u8,
    stinger_count: u8,
    dve_count: u8,
    supersource_count: u8,
    talkback_count: u8,
    sdi_count: u8,
    scalers_available: u8,
}

impl fmt::Display for Topology {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "M/Es: {}, Sources: {}, DSKs: {}, Aux: {}, Mix minus outputs: {}, Mediaplayers: {}, Multiviewers: {}, RS-485: {}, Hyperdecks: {}, Stingers: {}, DVEs: {}, Supersources: {}, Talkbacks: {}, SDIs: {}, Scalers: {}",
        self.me_count, self.source_count, self.dsk_count, self.aux_count, self.mixminus_output_count, self.mediaplayer_count, self.multiviewer_count, self.rs485_count,
        self.hyperdeck_count, self.stinger_count, self.dve_count, self.supersource_count, self.talkback_count, self.sdi_count, self.scalers_available)
    }
}

impl Topology {
    pub fn parse(data: &mut Bytes) -> Self {
        let me_count = data.get_u8();
        let source_count = data.get_u8();
        let dsk_count = data.get_u8();
        let aux_count = data.get_u8();
        let mixminus_output_count = data.get_u8();
        let mediaplayer_count = data.get_u8();
        let multiviewer_count = data.get_u8();
        let rs485_count = data.get_u8();
        let hyperdeck_count = data.get_u8();
        let dve_count = data.get_u8();
        let stinger_count = data.get_u8();
        let supersource_count = data.get_u8();
        data.get_u8(); // Unknown
        let talkback_count = data.get_u8();
        let sdi_count = data.get_u8(); // Not verified
        let scalers_available = data.get_u8(); // Not verified

        Topology {
            me_count,
            source_count,
            dsk_count,
            aux_count,
            mixminus_output_count,
            mediaplayer_count,
            multiviewer_count,
            rs485_count,
            hyperdeck_count,
            dve_count,
            stinger_count,
            supersource_count,
            talkback_count,
            sdi_count,
            scalers_available,
        }
    }
}

pub struct PowerState {
    primary: bool,
    secondary: bool,
}

impl PowerState {
    pub fn parse(data: &mut Bytes) -> Self {
        let states = data.get_u8();

        PowerState {
            primary: (states & 0x01) > 0,
            secondary: (states & 0x02) > 0,
        }
    }
}

impl fmt::Display for PowerState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Primary: {} Secondary: {}", self.primary, self.secondary)
    }
}

pub enum TimeCodeType {
    FreeRunning,
    TimeOfDay,
    Unknown(u8),
}

impl From<u8> for TimeCodeType {
    fn from(value: u8) -> Self {
        match value {
            0 => TimeCodeType::FreeRunning,
            1 => TimeCodeType::TimeOfDay,
            u => TimeCodeType::Unknown(u),
        }
    }
}

impl From<TimeCodeType> for u8 {
    fn from(value: TimeCodeType) -> Self {
        match value {
            TimeCodeType::FreeRunning => 0,
            TimeCodeType::TimeOfDay => 1,
            TimeCodeType::Unknown(u) => u,
        }
    }
}

impl fmt::Display for TimeCodeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TimeCodeType::FreeRunning => write!(f, "Free running"),
            TimeCodeType::TimeOfDay => write!(f, "Time of day"),
            TimeCodeType::Unknown(u) => write!(f, "Unknown time code type: {u}"),
        }
    }
}

pub struct TimeCodeState {
    timecode_type: TimeCodeType,
}

impl TimeCodeState {
    pub fn parse(data: &mut Bytes) -> Self {
        let timecode_type = data.get_u8();

        TimeCodeState {
            timecode_type: timecode_type.into(),
        }
    }
}

impl fmt::Display for TimeCodeState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.timecode_type)
    }
}

pub enum VideoMode {
    NTSC,
    PAL,
    NTSCWidescreen,
    PALWidescreen,
    Res720p50,
    Res720p59_94,
    Res1080i50,
    Res1080i59_94,
    Res1080p23_98,
    Res1080p24,
    Res1080p25,
    Res1080p29_97,
    Res1080p50,
    Res1080p59_94,
    Res4K23_98,
    Res4K24,
    Res4K25,
    Res4K29_97,
    Res4K50,
    Res4K59_94,
    Res8K23_98,
    Res8K24,
    Res8K25,
    Res8K29_97,
    Res8K50,
    Res8K59_94,
    Res1080p30,
    Res1080p60,
    Res720p60,
    Res1080i60,
    Unknown(u8),
}

impl VideoMode {
    pub fn parse(data: &mut Bytes) -> Self {
        data.get_u8().into()
    }
}

impl From<u8> for VideoMode {
    fn from(value: u8) -> Self {
        match value {
            0 => VideoMode::NTSC,
            1 => VideoMode::PAL,
            2 => VideoMode::NTSCWidescreen,
            3 => VideoMode::PALWidescreen,
            4 => VideoMode::Res720p50,
            5 => VideoMode::Res720p59_94,
            6 => VideoMode::Res1080i50,
            7 => VideoMode::Res1080i59_94,
            8 => VideoMode::Res1080p23_98,
            9 => VideoMode::Res1080p24,
            10 => VideoMode::Res1080p25,
            11 => VideoMode::Res1080p29_97,
            12 => VideoMode::Res1080p50,
            13 => VideoMode::Res1080p59_94,
            14 => VideoMode::Res4K23_98,
            15 => VideoMode::Res4K24,
            16 => VideoMode::Res4K25,
            17 => VideoMode::Res4K29_97,
            18 => VideoMode::Res4K50,
            19 => VideoMode::Res4K59_94,
            20 => VideoMode::Res8K23_98,
            21 => VideoMode::Res8K24,
            22 => VideoMode::Res8K25,
            23 => VideoMode::Res8K29_97,
            24 => VideoMode::Res8K50,
            25 => VideoMode::Res8K59_94,
            26 => VideoMode::Res1080p30,
            27 => VideoMode::Res1080p60,
            28 => VideoMode::Res720p60,
            29 => VideoMode::Res1080i60,
            u => VideoMode::Unknown(u),
        }
    }
}

impl From<VideoMode> for u8 {
    fn from(value: VideoMode) -> Self {
        match value {
            VideoMode::NTSC => 0,
            VideoMode::PAL => 1,
            VideoMode::NTSCWidescreen => 2,
            VideoMode::PALWidescreen => 3,
            VideoMode::Res720p50 => 4,
            VideoMode::Res720p59_94 => 5,
            VideoMode::Res1080i50 => 6,
            VideoMode::Res1080i59_94 => 7,
            VideoMode::Res1080p23_98 => 8,
            VideoMode::Res1080p24 => 9,
            VideoMode::Res1080p25 => 10,
            VideoMode::Res1080p29_97 => 11,
            VideoMode::Res1080p50 => 12,
            VideoMode::Res1080p59_94 => 13,
            VideoMode::Res4K23_98 => 14,
            VideoMode::Res4K24 => 15,
            VideoMode::Res4K25 => 16,
            VideoMode::Res4K29_97 => 17,
            VideoMode::Res4K50 => 18,
            VideoMode::Res4K59_94 => 19,
            VideoMode::Res8K23_98 => 20,
            VideoMode::Res8K24 => 21,
            VideoMode::Res8K25 => 22,
            VideoMode::Res8K29_97 => 23,
            VideoMode::Res8K50 => 24,
            VideoMode::Res8K59_94 => 25,
            VideoMode::Res1080p30 => 26,
            VideoMode::Res1080p60 => 27,
            VideoMode::Res720p60 => 28,
            VideoMode::Res1080i60 => 29,
            VideoMode::Unknown(u) => u,
        }
    }
}

impl fmt::Display for VideoMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VideoMode::NTSC => write!(f, "NTSC"),
            VideoMode::PAL => write!(f, "PAL"),
            VideoMode::NTSCWidescreen => write!(f, "NTSC widescreen"),
            VideoMode::PALWidescreen => write!(f, "PAL widescreen"),
            VideoMode::Res720p50 => write!(f, "720p50"),
            VideoMode::Res720p59_94 => write!(f, "720p59.94"),
            VideoMode::Res1080i50 => write!(f, "1080i50"),
            VideoMode::Res1080i59_94 => write!(f, "1080i59.94"),
            VideoMode::Res1080p23_98 => write!(f, "1080p23.98"),
            VideoMode::Res1080p24 => write!(f, "1080p24"),
            VideoMode::Res1080p25 => write!(f, "1080p25"),
            VideoMode::Res1080p29_97 => write!(f, "1080p29.97"),
            VideoMode::Res1080p50 => write!(f, "1080p50"),
            VideoMode::Res1080p59_94 => write!(f, "1080p59.94"),
            VideoMode::Res4K23_98 => write!(f, "4K23.98"),
            VideoMode::Res4K24 => write!(f, "4K24"),
            VideoMode::Res4K25 => write!(f, "4K25"),
            VideoMode::Res4K29_97 => write!(f, "4K29.97"),
            VideoMode::Res4K50 => write!(f, "4K50"),
            VideoMode::Res4K59_94 => write!(f, "4K59.94"),
            VideoMode::Res8K23_98 => write!(f, "8K23.98"),
            VideoMode::Res8K24 => write!(f, "8K24"),
            VideoMode::Res8K25 => write!(f, "8K25"),
            VideoMode::Res8K29_97 => write!(f, "8K29.97"),
            VideoMode::Res8K50 => write!(f, "8K50"),
            VideoMode::Res8K59_94 => write!(f, "8K59.94"),
            VideoMode::Res1080p30 => write!(f, "1080p30"),
            VideoMode::Res1080p60 => write!(f, "1080p60"),
            VideoMode::Res720p60 => write!(f, "720p60"),
            VideoMode::Res1080i60 => write!(f, "1080i60"),
            VideoMode::Unknown(u) => write!(f, "Unknown ({u})"),
        }
    }
}
