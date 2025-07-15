package main

// Keys

const ESCAPE string = "\x1b"
const ALT string = "\x1b"
// Adding escape in front of most key codes gives its alt variant.
// ALT can be used instead of ESCAPE for clarity.

const TAB string = "\x09"
const RETURN string = "\x0d"
const BACKSPACE string = "\x7f"
const SHIFT_TAB string = "\x1b[Z"
const CTRL_BACKSPACE string = "\x08"

var HOME = []string{"\x1b[1", "\x1b[H"}
var END = []string{"\x1b[4", "\x1b[F"}
const PG_UP string = "\x1b[5~"
const PG_DOWN string = "\x1b[6~"
const DELETE string = "\x1b[3~"
const INSERT string = "\x1b[2~"

// Arrows

const ARROW_UP string = "\x1b[A"
const ARROW_DOWN string = "\x1b[B"
const ARROW_LEFT string = "\x1b[D"
const ARROW_RIGHT string = "\x1b[C"
const SHIFT_ARROW_UP string = "\x1b[1;2A"
const SHIFT_ARROW_DOWN string = "\x1b[1;2B"
const SHIFT_ARROW_LEFT string = "\x1b[1;2D"
const SHIFT_ARROW_RIGHT string = "\x1b[1;2C"
const CTRL_ARROW_UP string = "\x1b[1;5A"
const CTRL_ARROW_DOWN string = "\x1b[1;5B"
const CTRL_ARROW_LEFT string = "\x1b[1;5D"
const CTRL_ARROW_RIGHT string = "\x1b[1;5C"
const CTRL_SHIFT_ARROW_UP string = "\x1b[1;6A"
const CTRL_SHIFT_ARROW_DOWN string = "\x1b[1;6B"
const CTRL_SHIFT_ARROW_LEFT string = "\x1b[1;6D"
const CTRL_SHIFT_ARROW_RIGHT string = "\x1b[1;6C"

// Function keys

var F1 = []string{"\x1bOP", "\x1b[11~"}
var F2 = []string{"\x1bOQ", "\x1b[12~"}
var F3 = []string{"\x1bOR", "\x1b[13~"}
var F4 = []string{"\x1bOS", "\x1b[14~"}
const F5 string = "\x1b[15~"
const F6 string = "\x1b[17~"
const F7 string = "\x1b[18~"
const F8 string = "\x1b[19~"
const F9 string = "\x1b[20~"
const F10 string = "\x1b[21~"
const F11 string = "\x1b[23~"
const F12 string = "\x1b[24~"

// Control + letter

const CTRL_A string = "\x01"
const CTRL_B string = "\x02"
const CTRL_C string = "\x03"
const CTRL_D string = "\x04"
const CTRL_E string = "\x05"
const CTRL_F string = "\x06"
const CTRL_G string = "\x07"
const CTRL_H string = "\x08"
const CTRL_I string = "\x09"
const CTRL_J string = "\x0a"
const CTRL_K string = "\x0b"
const CTRL_L string = "\x0c"
const CTRL_M string = "\x0d"
const CTRL_N string = "\x0e"
const CTRL_O string = "\x0f"
const CTRL_P string = "\x10"
const CTRL_Q string = "\x11"
const CTRL_R string = "\x12"
const CTRL_S string = "\x13"
const CTRL_T string = "\x14"
const CTRL_U string = "\x15"
const CTRL_V string = "\x16"
const CTRL_W string = "\x17"
const CTRL_X string = "\x18"
const CTRL_Y string = "\x19"
const CTRL_Z string = "\x1a"
