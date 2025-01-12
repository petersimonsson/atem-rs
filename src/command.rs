use std::fmt::Display;

use bytes::{Buf, Bytes};
use thiserror::Error;
use tracing::debug;

use crate::{
    parser::parse_str,
    source::Source,
    systeminfo::{PowerState, Topology, Version},
    tally::{TallyInputs, TallySources},
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
    TallySources(TallySources),
    PowerState(PowerState),
    TransitionStyleSelection(TransitionStyleSelection),
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
            b"TlSr" => {
                let tally_sources = TallySources::parse(&mut data);
                Ok(Command::TallySources(tally_sources))
            }
            b"Powr" => {
                let power_state = PowerState::parse(&mut data);
                Ok(Command::PowerState(power_state))
            }
            b"TrSS" => {
                let transition_style_selection = TransitionStyleSelection::parse(&mut data);
                Ok(Command::TransitionStyleSelection(
                    transition_style_selection,
                ))
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
            Command::TallySources(tallys) => write!(f, "Tally sources: {tallys}"),
            Command::PowerState(power) => write!(f, "Power state: {power}"),
            Command::TransitionStyleSelection(selection) => {
                write!(f, "Transition style selection: {selection}")
            }
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

pub enum TransitionStyle {
    Mix,
    Dip,
    Wipe,
    Dve,
    Stinger,
    Unknown(u8),
}

impl From<u8> for TransitionStyle {
    fn from(value: u8) -> Self {
        match value {
            0 => TransitionStyle::Mix,
            1 => TransitionStyle::Dip,
            2 => TransitionStyle::Wipe,
            3 => TransitionStyle::Dve,
            4 => TransitionStyle::Stinger,
            u => TransitionStyle::Unknown(u),
        }
    }
}

impl From<TransitionStyle> for u8 {
    fn from(value: TransitionStyle) -> Self {
        match value {
            TransitionStyle::Mix => 0,
            TransitionStyle::Dip => 1,
            TransitionStyle::Wipe => 2,
            TransitionStyle::Dve => 3,
            TransitionStyle::Stinger => 4,
            TransitionStyle::Unknown(u) => u,
        }
    }
}

impl Display for TransitionStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransitionStyle::Mix => write!(f, "Mix"),
            TransitionStyle::Dip => write!(f, "Dip"),
            TransitionStyle::Wipe => write!(f, "Wipe"),
            TransitionStyle::Dve => write!(f, "DVE"),
            TransitionStyle::Stinger => write!(f, "Stinger"),
            TransitionStyle::Unknown(u) => write!(f, "Unknown ({u})"),
        }
    }
}

pub struct TransitionStyleSelection {
    me: u8,
    current_style: TransitionStyle,
    current_selection: u8,
    next_style: TransitionStyle,
    next_selection: u8,
}

impl TransitionStyleSelection {
    pub fn parse(data: &mut Bytes) -> Self {
        let me = data.get_u8();
        let current_style = data.get_u8();
        let current_selection = data.get_u8();
        let next_style = data.get_u8();
        let next_selection = data.get_u8();

        TransitionStyleSelection {
            me,
            current_style: current_style.into(),
            current_selection,
            next_style: next_style.into(),
            next_selection,
        }
    }
}

impl Display for TransitionStyleSelection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ME: {} Current style: {} Current selection: {} Next style: {} Next selection: {}",
            self.me,
            self.current_style,
            self.current_selection,
            self.next_style,
            self.next_selection
        )
    }
}
