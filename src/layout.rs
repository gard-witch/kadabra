pub(crate) mod en;
pub(crate) mod ru;

use std::collections::HashMap;

use clap::ValueEnum;

use crate::layout::{en::EN, ru::RU};

#[rustfmt::skip]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub(crate) enum Key {
    GraveAccent, //First line, `
    Tilde, At, Hash, Dollar, Caret, Ampersand, //First line with Shift, ~ @ # $ ^ & 
    KeyQ, KeyW, KeyE, KeyR, KeyT, KeyY, KeyU, KeyI, KeyO, KeyP, LeftSquareBracket, RightSquareBracket, Backslash, // Second line ... [ ] \
    KeyShiftQ, KeyShiftW, KeyShiftE, KeyShiftR, KeyShiftT, KeyShiftY, KeyShiftU, KeyShiftI, KeyShiftO, KeyShiftP, LeftCurlyBrace, RightCurlyBrace, VerticalBar, // Second line with Shift ... { } |
    KeyA, KeyS, KeyD, KeyF, KeyG, KeyH, KeyJ, KeyK, KeyL, Semicolon, Apostrophe, // Third line ... ; '
    KeyShiftA, KeyShiftS, KeyShiftD, KeyShiftF, KeyShiftG, KeyShiftH, KeyShiftJ, KeyShiftK, KeyShiftL, Colon, QuotationMark, // Third line with Shift ... : '
    KeyZ, KeyX, KeyC, KeyV, KeyB, KeyN, KeyM, Comma, Period, Slash, //Fourth line ... , . /
    KeyShiftZ, KeyShiftX, KeyShiftC, KeyShiftV, KeyShiftB, KeyShiftN, KeyShiftM, LessThanSign, GreaterThanSign, QuestionMark, //Fourth line with Shift, ... < > ?
}

#[derive(Default)]
pub(crate) struct Layout {
    to_keys: HashMap<char, Key>,
    to_chars: HashMap<Key, char>,
}

impl Layout {
    fn insert(&mut self, key: Key, ch: char) {
        self.to_keys.insert(ch, key);
        self.to_chars.insert(key, ch);
    }

    pub(crate) fn get_by_key(&self, key: Key) -> char {
        *self.to_chars.get(&key).unwrap()
    }
    pub(crate) fn get_by_char(&self, key: char) -> Option<Key> {
        self.to_keys.get(&key).copied()
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub(crate) enum LayoutKind {
    En,
    Ru,
}

impl From<LayoutKind> for &Layout {
    fn from(value: LayoutKind) -> Self {
        match value {
            LayoutKind::En => &EN,
            LayoutKind::Ru => &RU,
        }
    }
}
