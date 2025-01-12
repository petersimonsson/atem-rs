use std::fmt::Display;

use bytes::{Buf, Bytes};

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

pub struct SourceTally {
    source_id: u16,
    state: TallyState,
}

impl SourceTally {
    pub fn new(source_id: u16, state: TallyState) -> Self {
        SourceTally { source_id, state }
    }
}

impl Display for SourceTally {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Source: {} {}", self.source_id, self.state)
    }
}

pub struct TallySources {
    tally_states: Vec<SourceTally>,
}

impl TallySources {
    pub fn parse(data: &mut Bytes) -> Self {
        let count = data.get_u16();
        let mut tally_states: Vec<SourceTally> = Vec::default();

        for _ in 0..count {
            let source_id = data.get_u16();
            let byte = data.get_u8();
            tally_states.push(SourceTally::new(
                source_id,
                TallyState::new((byte & 0x01) > 0, (byte & 0x02) > 0),
            ));
        }

        TallySources { tally_states }
    }
}

impl Display for TallySources {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let state_str = self
            .tally_states
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        write!(f, "{}", state_str)
    }
}
