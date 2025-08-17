// License: https://unlicense.org/
// Source:  https://github.com/jackiboi307/escape-sequences

#![allow(dead_code)] // disable warnings for unused code

// Keys

pub const ESCAPE: &'static str = "\x1b";
pub const ALT: &'static str = "\x1b";
// Adding escape in front of most key codes gives its alt variant.
// ALT can be used instead of ESCAPE for clarity.

pub const TAB: &'static str = "\x09";
pub const RETURN: &'static str = "\x0d";
pub const BACKSPACE: &'static str = "\x7f";
pub const SHIFT_TAB: &'static str = "\x1b[Z";
pub const CTRL_BACKSPACE: &'static str = "\x08";

pub const HOME: [&'static str; 2] = ["\x1b[1", "\x1b[H"];
pub const END: [&'static str; 2] = ["\x1b[4", "\x1b[F"];
pub const PG_UP: &'static str = "\x1b[5~";
pub const PG_DOWN: &'static str = "\x1b[6~";
pub const DELETE: &'static str = "\x1b[3~";
pub const INSERT: &'static str = "\x1b[2~";

// Arrows

pub const ARROW_UP: &'static str = "\x1b[A";
pub const ARROW_DOWN: &'static str = "\x1b[B";
pub const ARROW_LEFT: &'static str = "\x1b[D";
pub const ARROW_RIGHT: &'static str = "\x1b[C";
pub const SHIFT_ARROW_UP: &'static str = "\x1b[1;2A";
pub const SHIFT_ARROW_DOWN: &'static str = "\x1b[1;2B";
pub const SHIFT_ARROW_LEFT: &'static str = "\x1b[1;2D";
pub const SHIFT_ARROW_RIGHT: &'static str = "\x1b[1;2C";
pub const CTRL_ARROW_UP: &'static str = "\x1b[1;5A";
pub const CTRL_ARROW_DOWN: &'static str = "\x1b[1;5B";
pub const CTRL_ARROW_LEFT: &'static str = "\x1b[1;5D";
pub const CTRL_ARROW_RIGHT: &'static str = "\x1b[1;5C";
pub const CTRL_SHIFT_ARROW_UP: &'static str = "\x1b[1;6A";
pub const CTRL_SHIFT_ARROW_DOWN: &'static str = "\x1b[1;6B";
pub const CTRL_SHIFT_ARROW_LEFT: &'static str = "\x1b[1;6D";
pub const CTRL_SHIFT_ARROW_RIGHT: &'static str = "\x1b[1;6C";

// Function keys

pub const F1: [&'static str; 2] = ["\x1bOP", "\x1b[11~"];
pub const F2: [&'static str; 2] = ["\x1bOQ", "\x1b[12~"];
pub const F3: [&'static str; 2] = ["\x1bOR", "\x1b[13~"];
pub const F4: [&'static str; 2] = ["\x1bOS", "\x1b[14~"];
pub const F5: &'static str = "\x1b[15~";
pub const F6: &'static str = "\x1b[17~";
pub const F7: &'static str = "\x1b[18~";
pub const F8: &'static str = "\x1b[19~";
pub const F9: &'static str = "\x1b[20~";
pub const F10: &'static str = "\x1b[21~";
pub const F11: &'static str = "\x1b[23~";
pub const F12: &'static str = "\x1b[24~";

// Control + letter

pub const CTRL_A: &'static str = "\x01";
pub const CTRL_B: &'static str = "\x02";
pub const CTRL_C: &'static str = "\x03";
pub const CTRL_D: &'static str = "\x04";
pub const CTRL_E: &'static str = "\x05";
pub const CTRL_F: &'static str = "\x06";
pub const CTRL_G: &'static str = "\x07";
pub const CTRL_H: &'static str = "\x08";
pub const CTRL_I: &'static str = "\x09";
pub const CTRL_J: &'static str = "\x0a";
pub const CTRL_K: &'static str = "\x0b";
pub const CTRL_L: &'static str = "\x0c";
pub const CTRL_M: &'static str = "\x0d";
pub const CTRL_N: &'static str = "\x0e";
pub const CTRL_O: &'static str = "\x0f";
pub const CTRL_P: &'static str = "\x10";
pub const CTRL_Q: &'static str = "\x11";
pub const CTRL_R: &'static str = "\x12";
pub const CTRL_S: &'static str = "\x13";
pub const CTRL_T: &'static str = "\x14";
pub const CTRL_U: &'static str = "\x15";
pub const CTRL_V: &'static str = "\x16";
pub const CTRL_W: &'static str = "\x17";
pub const CTRL_X: &'static str = "\x18";
pub const CTRL_Y: &'static str = "\x19";
pub const CTRL_Z: &'static str = "\x1a";
