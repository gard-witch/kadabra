mod en;
mod ru;

use std::collections::HashMap;

#[rustfmt::skip]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
enum Key {
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
struct Layout {
    to_keys: HashMap<char, Key>,
    to_chars: HashMap<Key, char>,
}

impl Layout {
    fn insert(&mut self, key: Key, ch: char) {
        self.to_keys.insert(ch, key);
        self.to_chars.insert(key, ch);
    }
}
