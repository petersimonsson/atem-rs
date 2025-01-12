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
    color_gen_count: u8,
    aux_count: u8,
    dsk_count: u8,
    key_count: u8,
    stinger_count: u8,
    dve_count: u8,
    supersource_count: u8,
    has_sd: bool,
}

impl fmt::Display for Topology {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "M/Es: {}, Sources: {}, Color generators: {}, Aux: {}, DSKs: {}, Keys: {}, Stingers: {}, DVEs: {}, SuperSources: {}, Has SD: {}",
        self.me_count, self.source_count, self.color_gen_count, self.aux_count, self.dsk_count, self.key_count, self.stinger_count, self.dve_count,
        self.supersource_count, self.has_sd)
    }
}

impl Topology {
    // TODO: Figure out if the parsing is correct
    pub fn parse(data: &mut Bytes) -> Self {
        let me_count = data.get_u8();
        let source_count = data.get_u8();
        let color_gen_count = data.get_u8();
        let aux_count = data.get_u8();
        data.get_u8(); // Unknown
        let dsk_count = data.get_u8();
        data.get_u8(); // Unknown
        let key_count = data.get_u8();
        let stinger_count = data.get_u8();
        let dve_count = data.get_u8();
        data.get_u8(); // Unknown
        let supersource_count = data.get_u8();
        data.get_u8(); // Unknown
        let has_sd = data.get_u8() != 0;

        Topology {
            me_count,
            source_count,
            color_gen_count,
            aux_count,
            dsk_count,
            key_count,
            stinger_count,
            dve_count,
            supersource_count,
            has_sd,
        }
    }
}
