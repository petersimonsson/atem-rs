use std::fmt::Display;

use bytes::{Buf, Bytes};
use thiserror::Error;
use tracing::debug;

use crate::{
    parser::parse_str,
    source::Source,
    systeminfo::{Topology, Version},
};

#[derive(Debug, Error)]
pub enum Error {
    #[error("String parsing failed")]
    Utf8Error(#[from] std::string::FromUtf8Error),
    #[error("Unknown command ({0})")]
    UnknownCommand(String),
}

#[allow(dead_code)]
pub enum Command {
    Version(Version),
    Product(String),
    Topology(Topology),
    Source(Source),
    ProgramInput(SourceSelection),
    PreviewInput(SourceSelection),
    TransitionPosition(TransitionPosition),
    Time(Time),
    TallyInputs(TallyInputs),
}

impl Command {
    pub fn parse(payload: &mut Bytes) -> Result<Command, Error> {
        let size = payload.get_u16();
        payload.get_u16(); // skip two bytes, unknow function.
        let cmd = payload.split_to(4);
        let data_size = size as usize - 8;
        let mut data = payload.split_to(data_size);
        debug!("Command {:?} Size: {}", cmd, size);

        match &cmd[..] {
            b"_ver" => {
                let version = Version::parse(&mut data);
                Ok(Command::Version(version))
            }
            b"_pin" => {
                let product = parse_str(&mut data)?.unwrap();
                Ok(Command::Product(product))
            }
            b"_top" => {
                let topology = Topology::parse(&mut data);
                Ok(Command::Topology(topology))
            }
            b"InPr" => {
                let source = Source::parse(&mut data)?;
                Ok(Command::Source(source))
            }
            b"PrgI" => {
                let source_selection = SourceSelection::parse(&mut data);
                Ok(Command::ProgramInput(source_selection))
            }
            b"PrvI" => {
                let source_selection = SourceSelection::parse(&mut data);
                Ok(Command::PreviewInput(source_selection))
            }
            b"TrPs" => {
                let transition_position = TransitionPosition::parse(&mut data);
                Ok(Command::TransitionPosition(transition_position))
            }
            b"Time" => {
                let time = Time::parse(&mut data);
                Ok(Command::Time(time))
            }
            b"TlIn" => {
                let tally_inputs = TallyInputs::parse(&mut data);
                Ok(Command::TallyInputs(tally_inputs))
            }
            _ => {
                debug!(
                    "Unknown command: {} Data: {:02X?} [{}]",
                    String::from_utf8(cmd.to_vec())?,
                    &data[..],
                    data_size
                );
                Err(Error::UnknownCommand(String::from_utf8(cmd.to_vec())?))
            }
        }
    }
}

impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Command::Version(version) => write!(f, "Firmware version: {version}"),
            Command::Product(product) => write!(f, "Product: {product}"),
            Command::Topology(topology) => write!(f, "Topology: {topology}"),
            Command::Source(source) => write!(f, "{source}"),
            Command::ProgramInput(selection) => write!(f, "Program input: {selection}"),
            Command::PreviewInput(selection) => write!(f, "Preview input: {selection}"),
            Command::TransitionPosition(position) => write!(f, "Transition position: {position}"),
            Command::Time(time) => write!(f, "Time: {time}"),
            Command::TallyInputs(tallys) => write!(f, "Tally inputs: {tallys}"),
        }
    }
}

pub struct SourceSelection {
    me: u8,
    source_id: u16,
}

impl SourceSelection {
    pub fn parse(data: &mut Bytes) -> Self {
        let me = data.get_u8();
        data.get_u8(); // Skip
        let source_id = data.get_u16();

        SourceSelection { me, source_id }
    }
}

impl Display for SourceSelection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ME: {} Source: {}", self.me, self.source_id)
    }
}

pub struct TransitionPosition {
    me: u8,
    frame_count: u8,
    position: u16,
}

impl TransitionPosition {
    pub fn parse(data: &mut Bytes) -> Self {
        let me = data.get_u8();
        data.get_u8(); // Skip
        let frame_count = data.get_u8();
        data.get_u8(); // Skip
        let position = data.get_u16();

        TransitionPosition {
            me,
            frame_count,
            position,
        }
    }
}

impl Display for TransitionPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ME: {} Frame count: {} Position: {}",
            self.me, self.frame_count, self.position
        )
    }
}

pub struct Time {
    hour: u8,
    minute: u8,
    second: u8,
    frame: u8,
}

impl Time {
    pub fn parse(data: &mut Bytes) -> Self {
        let hour = data.get_u8();
        let minute = data.get_u8();
        let second = data.get_u8();
        let frame = data.get_u8();
        Time {
            hour,
            minute,
            second,
            frame,
        }
    }
}

impl Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:02}:{:02}:{:02}:{:02}",
            self.hour, self.minute, self.second, self.frame
        )
    }
}

#[derive(Default, Debug)]
pub struct TallyState {
    program: bool,
    preview: bool,
}

impl Display for TallyState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Program: {} Preview: {}", self.program, self.preview)
    }
}

impl TallyState {
    pub fn new(program: bool, preview: bool) -> Self {
        TallyState { program, preview }
    }
}

pub struct TallyInputs {
    tally_states: Vec<TallyState>,
}

impl TallyInputs {
    pub fn parse(data: &mut Bytes) -> Self {
        let count = data.get_u16();
        let mut tally_states: Vec<TallyState> = Vec::default();

        for _ in 0..count {
            let byte = data.get_u8();
            tally_states.push(TallyState::new((byte & 0x01) > 0, (byte & 0x02) > 0));
        }

        TallyInputs { tally_states }
    }
}

impl Display for TallyInputs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let state_str = self
            .tally_states
            .iter()
            .enumerate()
            .map(|(index, state)| format!("Input: {} State: {}", index + 1, state))
            .collect::<Vec<String>>()
            .join(", ");
        write!(f, "{}", state_str)
    }
}
