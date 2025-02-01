use std::fmt::Display;

use bytes::{Buf, Bytes};
use thiserror::Error;
use tracing::debug;

use crate::{
    multiview::{MultiViewInput, MultiViewLayout, MultiViewSafeArea, MultiViewVU},
    parser::parse_str,
    source::Source,
    systeminfo::{
        MeConfig, MediaPlayerConfig, PowerState, TimeCodeState, Topology, Version, VideoMode,
        VideoModeConfig,
    },
    tally::{TallyInputs, TallySources},
    transition::{
        TransitionDVE, TransitionDip, TransitionMix, TransitionPreview, TransitionStinger,
        TransitionStyleSelection, TransitionWipe,
    },
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
    AuxSource(SourceSelection),
    MultiViewInput(MultiViewInput),
    TimeCodeState(TimeCodeState),
    VideoMode(VideoMode),
    MeConfig(MeConfig),
    MediaPlayerConfig(MediaPlayerConfig),
    VideoModeConfig(VideoModeConfig),
    MultiViewVU(MultiViewVU),
    MultiViewSafeArea(MultiViewSafeArea),
    MultiViewLayout(MultiViewLayout),
    TransitionPreview(TransitionPreview),
    TransitionMix(TransitionMix),
    TransitionDip(TransitionDip),
    TransitionWipe(TransitionWipe),
    TransitionDVE(TransitionDVE),
    TransitionStinger(TransitionStinger),
    KeyerOnAir(KeyerOnAir),
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
            b"AuxS" => {
                let source_selection = SourceSelection::parse(&mut data);
                Ok(Command::AuxSource(source_selection))
            }
            b"MvIn" => {
                let multiview_input = MultiViewInput::parse(&mut data);
                Ok(Command::MultiViewInput(multiview_input))
            }
            b"TCCc" => {
                let timecode_state = TimeCodeState::parse(&mut data);
                Ok(Command::TimeCodeState(timecode_state))
            }
            b"VidM" => {
                let videomode = VideoMode::parse(&mut data);
                Ok(Command::VideoMode(videomode))
            }
            b"_MeC" => {
                let me_config = MeConfig::parse(&mut data);
                Ok(Command::MeConfig(me_config))
            }
            b"_mpl" => {
                let media_player_config = MediaPlayerConfig::parse(&mut data);
                Ok(Command::MediaPlayerConfig(media_player_config))
            }
            b"_VMC" => {
                let videomode_config = VideoModeConfig::parse(&mut data);
                Ok(Command::VideoModeConfig(videomode_config))
            }
            b"VuMC" => {
                let multiview_vu = MultiViewVU::parse(&mut data);
                Ok(Command::MultiViewVU(multiview_vu))
            }
            b"SaMw" => {
                let multiview_safe_area = MultiViewSafeArea::parse(&mut data);
                Ok(Command::MultiViewSafeArea(multiview_safe_area))
            }
            b"MvPr" => {
                let multiview_layout = MultiViewLayout::parse(&mut data);
                Ok(Command::MultiViewLayout(multiview_layout))
            }
            b"TrPr" => {
                let transition_preview = TransitionPreview::parse(&mut data);
                Ok(Command::TransitionPreview(transition_preview))
            }
            b"TMxP" => {
                let transition_mix = TransitionMix::parse(&mut data);
                Ok(Command::TransitionMix(transition_mix))
            }
            b"TDpP" => {
                let transition_dip = TransitionDip::parse(&mut data);
                Ok(Command::TransitionDip(transition_dip))
            }
            b"TWpP" => {
                let transition_wipe = TransitionWipe::parse(&mut data);
                Ok(Command::TransitionWipe(transition_wipe))
            }
            b"TDvP" => {
                let transtion_dve = TransitionDVE::parse(&mut data);
                Ok(Command::TransitionDVE(transtion_dve))
            }
            b"TStP" => {
                let transition_stinger = TransitionStinger::parse(&mut data);
                Ok(Command::TransitionStinger(transition_stinger))
            }
            b"KeOn" => {
                let keyer_on_air = KeyerOnAir::parse(&mut data);
                Ok(Command::KeyerOnAir(keyer_on_air))
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
            Command::ProgramInput(selection) => write!(f, "Program input ME: {selection}"),
            Command::PreviewInput(selection) => write!(f, "Preview input ME: {selection}"),
            Command::TransitionPosition(position) => write!(f, "Transition position: {position}"),
            Command::Time(time) => write!(f, "Time: {time}"),
            Command::TallyInputs(tallys) => write!(f, "Tally inputs: {tallys}"),
            Command::TallySources(tallys) => write!(f, "Tally sources: {tallys}"),
            Command::PowerState(power) => write!(f, "Power state: {power}"),
            Command::TransitionStyleSelection(selection) => {
                write!(f, "Transition style selection: {selection}")
            }
            Command::AuxSource(selection) => write!(f, "Aux: {selection}"),
            Command::MultiViewInput(input) => write!(f, "Multiview input: {input}"),
            Command::TimeCodeState(state) => write!(f, "Time code state: {state}"),
            Command::VideoMode(mode) => write!(f, "Video mode: {mode}"),
            Command::MeConfig(config) => write!(f, "ME config: {config}"),
            Command::MediaPlayerConfig(config) => write!(f, "Media player config: {config}"),
            Command::VideoModeConfig(config) => write!(f, "Video modes: {config}"),
            Command::MultiViewVU(vu) => write!(f, "Multiview VU: {vu}"),
            Command::MultiViewSafeArea(safe_area) => write!(f, "Multiview safe area: {safe_area}"),
            Command::MultiViewLayout(layout) => write!(f, "Multiview layout: {layout}"),
            Command::TransitionPreview(preview) => write!(f, "Transition preview: {preview}"),
            Command::TransitionMix(mix) => write!(f, "Transition mix: {mix}"),
            Command::TransitionDip(dip) => write!(f, "Transition dip: {dip}"),
            Command::TransitionWipe(wipe) => write!(f, "Transition wipe: {wipe}"),
            Command::TransitionDVE(dve) => write!(f, "Transition DVE: {dve}"),
            Command::TransitionStinger(stinger) => write!(f, "Transition stinger: {stinger}"),
            Command::KeyerOnAir(on_air) => write!(f, "Keyer on air: {on_air}"),
        }
    }
}

pub struct SourceSelection {
    destination: u8,
    source_id: u16,
}

impl SourceSelection {
    pub fn parse(data: &mut Bytes) -> Self {
        let destination = data.get_u8();
        data.get_u8(); // Skip
        let source_id = data.get_u16();

        SourceSelection {
            destination,
            source_id,
        }
    }
}

impl Display for SourceSelection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} Source: {}", self.destination, self.source_id)
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
