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
