use std::fmt::Display;

use bytes::{Buf, Bytes};

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

pub struct TransitionPreview {
    me: u8,
    enabled: bool,
}

impl TransitionPreview {
    pub fn parse(data: &mut Bytes) -> Self {
        let me = data.get_u8();
        let enabled = data.get_u8() == 1;

        Self { me, enabled }
    }
}

impl Display for TransitionPreview {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ME: {} Enabled: {}", self.me, self.enabled)
    }
}

pub struct TransitionMix {
    me: u8,
    rate: u8,
}

impl TransitionMix {
    pub fn parse(data: &mut Bytes) -> Self {
        let me = data.get_u8();
        let rate = data.get_u8();

        Self { me, rate }
    }
}

impl Display for TransitionMix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ME: {} Rate: {}", self.me, self.rate)
    }
}

pub struct TransitionDip {
    me: u8,
    rate: u8,
    source: u16,
}

impl TransitionDip {
    pub fn parse(data: &mut Bytes) -> Self {
        let me = data.get_u8();
        let rate = data.get_u8();
        let source = data.get_u16();

        Self { me, rate, source }
    }
}

impl Display for TransitionDip {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ME: {} Rate: {} Source: {}",
            self.me, self.rate, self.source
        )
    }
}

pub struct TransitionWipe {
    me: u8,
    rate: u8,
    pattern: u8,
    border_width: u16,
    border_fill_source: u16,
    symmetry: u16,
    softness: u16,
    origin_x: u16,
    origin_y: u16,
    reverse: bool,
    flip: bool,
}

impl TransitionWipe {
    pub fn parse(data: &mut Bytes) -> Self {
        let me = data.get_u8();
        let rate = data.get_u8();
        let pattern = data.get_u8();
        data.get_u8(); // Unknown
        let border_width = data.get_u16();
        let border_fill_source = data.get_u16();
        let symmetry = data.get_u16();
        let softness = data.get_u16();
        let origin_x = data.get_u16();
        let origin_y = data.get_u16();
        let reverse = data.get_u8() == 1;
        let flip = data.get_u8() == 1;

        Self {
            me,
            rate,
            pattern,
            border_width,
            border_fill_source,
            symmetry,
            softness,
            origin_x,
            origin_y,
            reverse,
            flip,
        }
    }
}

impl Display for TransitionWipe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ME: {} Rate: {} Pattern: {} Border width: {} Border fill source: {} Symmetry: {} Softness {} Origin X: {} Origin Y: {} Reverse: {} Flip: {}",
            self.me, self.rate, self.pattern, self.border_width, self.border_fill_source, self.symmetry,
            self.softness, self.origin_x, self.origin_y, self.reverse, self.flip)
    }
}

pub struct TransitionDVE {
    me: u8,
    rate: u8,
    style: u8,
    fill_source: u16,
    key_source: u16,
    key_enabled: bool,
    key_premultiplied: bool,
    key_clip: u16,
    key_gain: u16,
    key_invert: bool,
    reverse: bool,
    flip: bool,
}

impl TransitionDVE {
    pub fn parse(data: &mut Bytes) -> Self {
        let me = data.get_u8();
        let rate = data.get_u8();
        data.get_u8(); // Unknown
        let style = data.get_u8();
        let fill_source = data.get_u16();
        let key_source = data.get_u16();
        let key_enabled = data.get_u8() == 1;
        let key_premultiplied = data.get_u8() == 1;
        let key_clip = data.get_u16();
        let key_gain = data.get_u16();
        let key_invert = data.get_u8() == 1;
        let reverse = data.get_u8() == 1;
        let flip = data.get_u8() == 1;

        Self {
            me,
            rate,
            style,
            fill_source,
            key_source,
            key_enabled,
            key_premultiplied,
            key_clip,
            key_gain,
            key_invert,
            reverse,
            flip,
        }
    }
}

impl Display for TransitionDVE {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ME: {} Rate: {} Style: {} Fill source: {} Key Source: {} Key enabled: {} Key premultiplied: {} Key clip: {} Key gain: {} Key invert: {} Reverse: {} Flip: {}",
            self.me, self.rate, self.style, self.fill_source, self.key_source, self.key_enabled, self.key_premultiplied,
            self.key_clip, self.key_gain, self.key_invert, self.reverse, self.flip)
    }
}

pub struct TransitionStinger {
    me: u8,
    source: u16,
    key_premultiplied: bool,
    key_clip: u16,
    key_gain: u16,
    key_invert: bool,
    pre_roll: u16,
    clip_duration: u16,
    rate: u16,
}

impl TransitionStinger {
    pub fn parse(data: &mut Bytes) -> Self {
        // TODO: Verify that this is correct
        let me = data.get_u8();
        let source = data.get_u16();
        let key_premultiplied = data.get_u8() == 1;
        let key_clip = data.get_u16();
        let key_gain = data.get_u16();
        let key_invert = data.get_u8() == 1;
        let pre_roll = data.get_u16();
        let clip_duration = data.get_u16();
        let rate = data.get_u16();

        Self {
            me,
            source,
            key_premultiplied,
            key_clip,
            key_gain,
            key_invert,
            pre_roll,
            clip_duration,
            rate,
        }
    }
}

impl Display for TransitionStinger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ME: {} Source: {} Key premultiplied: {} Key clip: {} Key gain: {} Key invert: {} Pre-roll: {} Clip duration: {} Rate: {}",
            self.me, self.source, self.key_premultiplied, self.key_clip, self.key_gain, self.key_invert,
            self.pre_roll, self.clip_duration, self.rate)
    }
}
