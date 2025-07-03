#![allow(dead_code)] // disable warnings for unused code

// Keys

const ESCAPE:                 &'static str = "\x1b";
const ALT_MOD:                &'static str = "\x1b";
// Adding escape in front of most key codes gives its alt variant.
// ALT_MOD can be used instead of ESCAPE for clarity.

const TAB:                    &'static str = "\x09";
const RETURN:                 &'static str = "\x0d";
const BACKSPACE:              &'static str = "\x7f";
const SHIFT_TAB:              &'static str = "\x1b[Z";
const CTRL_BACKSPACE:         &'static str = "\x08";

const HOME:                   [&'static str; 2] = ["\x1b[1", "\x1b[H"];
const END:                    [&'static str; 2] = ["\x1b[4", "\x1b[F"];
const PG_UP:                  &'static str = "\x1b[5~";
const PG_DOWN:                &'static str = "\x1b[6~";
const DELETE:                 &'static str = "\x1b[3~";
const INSERT:                 &'static str = "\x1b[2~";

// Arrows

const ARROW_UP:               &'static str = "\x1b[A";
const ARROW_DOWN:             &'static str = "\x1b[B";
const ARROW_LEFT:             &'static str = "\x1b[D";
const ARROW_RIGHT:            &'static str = "\x1b[C";
const SHIFT_ARROW_UP:         &'static str = "\x1b[1;2A";
const SHIFT_ARROW_DOWN:       &'static str = "\x1b[1;2B";
const SHIFT_ARROW_LEFT:       &'static str = "\x1b[1;2D";
const SHIFT_ARROW_RIGHT:      &'static str = "\x1b[1;2C";
const CTRL_ARROW_UP:          &'static str = "\x1b[1;5A";
const CTRL_ARROW_DOWN:        &'static str = "\x1b[1;5B";
const CTRL_ARROW_LEFT:        &'static str = "\x1b[1;5D";
const CTRL_ARROW_RIGHT:       &'static str = "\x1b[1;5C";
const CTRL_SHIFT_ARROW_UP:    &'static str = "\x1b[1;6A";
const CTRL_SHIFT_ARROW_DOWN:  &'static str = "\x1b[1;6B";
const CTRL_SHIFT_ARROW_LEFT:  &'static str = "\x1b[1;6D";
const CTRL_SHIFT_ARROW_RIGHT: &'static str = "\x1b[1;6C";

// Function keys

const F1:                     [&'static str; 2] = ["\x1bOP", "\x1b[11~"];
const F2:                     [&'static str; 2] = ["\x1bOQ", "\x1b[12~"];
const F3:                     [&'static str; 2] = ["\x1bOR", "\x1b[13~"];
const F4:                     [&'static str; 2] = ["\x1bOS", "\x1b[14~"];
const F5:                     &'static str = "\x1b[15~";
const F6:                     &'static str = "\x1b[17~";
const F7:                     &'static str = "\x1b[18~";
const F8:                     &'static str = "\x1b[19~";
const F9:                     &'static str = "\x1b[20~";
const F10:                    &'static str = "\x1b[21~";
const F11:                    &'static str = "\x1b[23~";
const F12:                    &'static str = "\x1b[24~";

// Control + letter

const CTRL_A:                 &'static str = "\x01";
const CTRL_B:                 &'static str = "\x02";
const CTRL_C:                 &'static str = "\x03";
const CTRL_D:                 &'static str = "\x04";
const CTRL_E:                 &'static str = "\x05";
const CTRL_F:                 &'static str = "\x06";
const CTRL_G:                 &'static str = "\x07";
const CTRL_H:                 &'static str = "\x08";
const CTRL_I:                 &'static str = "\x09";
const CTRL_J:                 &'static str = "\x0a";
const CTRL_K:                 &'static str = "\x0b";
const CTRL_L:                 &'static str = "\x0c";
const CTRL_M:                 &'static str = "\x0d";
const CTRL_N:                 &'static str = "\x0e";
const CTRL_O:                 &'static str = "\x0f";
const CTRL_P:                 &'static str = "\x10";
const CTRL_Q:                 &'static str = "\x11";
const CTRL_R:                 &'static str = "\x12";
const CTRL_S:                 &'static str = "\x13";
const CTRL_T:                 &'static str = "\x14";
const CTRL_U:                 &'static str = "\x15";
const CTRL_V:                 &'static str = "\x16";
const CTRL_W:                 &'static str = "\x17";
const CTRL_X:                 &'static str = "\x18";
const CTRL_Y:                 &'static str = "\x19";
const CTRL_Z:                 &'static str = "\x1a";
