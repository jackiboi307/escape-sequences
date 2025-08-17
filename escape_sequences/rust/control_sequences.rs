// License: https://unlicense.org/
// Source:  https://github.com/jackiboi307/escape-sequences

#![allow(dead_code)] // disable warnings for unused code

// Control codes

pub const BELL: &'static str = "\x07";

// Cursor control

pub const CUR_HOME: &'static str = "\x1b[H";
pub const CUR_SET: &'static str = "\x1b[{};{}H";
pub const CUR_UP: &'static str = "\x1b[{}A";
pub const CUR_DOWN: &'static str = "\x1b[{}B";
pub const CUR_RIGHT: &'static str = "\x1b[{}C";
pub const CUR_LEFT: &'static str = "\x1b[{}D";
pub const CUR_DOWN_BEG: &'static str = "\x1b[{}E";
pub const CUR_UP_BEG: &'static str = "\x1b[{}F";
pub const CUR_COL: &'static str = "\x1b[{}G";

pub const CUR_UP_ONE: &'static str = "\x1b[1A";
pub const CUR_DOWN_ONE: &'static str = "\x1b[1B";
pub const CUR_RIGHT_ONE: &'static str = "\x1b[1C";
pub const CUR_LEFT_ONE: &'static str = "\x1b[1D";

pub const CUR_COL_HOME: &'static str = "\x1b[0G";

pub const CUR_HIDE: &'static str = "\x1b[?25l";
pub const CUR_SHOW: &'static str = "\x1b[?25h";

pub const CUR_SAVE: &'static str = "\x1b7";
pub const CUR_RESTORE: &'static str = "\x1b8";
pub const CUR_SAVE_SCO: &'static str = "\x1b[s";
pub const CUR_RESTORE_SCO: &'static str = "\x1b[u";

// Screen operations

pub const SCREEN_SAVE: &'static str = "\x1b[?47h";
pub const RESTORE_SAVE: &'static str = "\x1b[?47l";

pub const ERASE_SCREEN: &'static str = "\x1b[2J";
pub const ERASE_LINE: &'static str = "\x1b[2K";
pub const ERASE_TO_END: &'static str = "\x1b[0J";
pub const ERASE_TO_LINE_END: &'static str = "\x1b[0J";

// Style

pub const RESET: &'static str = "\x1b[0m";
pub const BOLD: &'static str = "\x1b[1m";
pub const DIM: &'static str = "\x1b[2m";
pub const ITALIC: &'static str = "\x1b[3m";
pub const UNDERLINE: &'static str = "\x1b[4m";
pub const BLINK: &'static str = "\x1b[5m";
pub const STRIKE: &'static str = "\x1b[9m";

// 256 colors

pub const FG_ID: &'static str = "\x1b[38;5;{}m";
pub const BG_ID: &'static str = "\x1b[48;5;{}m";

// True color

pub const FG_RGB: &'static str = "\x1b[38;2;{};{};{}m";
pub const BG_RGB: &'static str = "\x1b[48;2;{};{};{}m";

// 16 color codes

pub const FG_BLACK: &'static str = "\x1b[30m";
pub const FG_RED: &'static str = "\x1b[31m";
pub const FG_GREEN: &'static str = "\x1b[32m";
pub const FG_YELLOW: &'static str = "\x1b[33m";
pub const FG_BLUE: &'static str = "\x1b[34m";
pub const FG_MAGENTA: &'static str = "\x1b[35m";
pub const FG_CYAN: &'static str = "\x1b[36m";
pub const FG_WHITE: &'static str = "\x1b[37m";
pub const FG_DEFAULT: &'static str = "\x1b[39m";

pub const BG_BLACK: &'static str = "\x1b[40m";
pub const BG_RED: &'static str = "\x1b[41m";
pub const BG_GREEN: &'static str = "\x1b[42m";
pub const BG_YELLOW: &'static str = "\x1b[43m";
pub const BG_BLUE: &'static str = "\x1b[44m";
pub const BG_MAGENTA: &'static str = "\x1b[45m";
pub const BG_CYAN: &'static str = "\x1b[46m";
pub const BG_WHITE: &'static str = "\x1b[47m";
pub const BG_DEFAULT: &'static str = "\x1b[49m";

// Bright versions

pub const FG_BLACK_B: &'static str = "\x1b[90m";
pub const FG_RED_B: &'static str = "\x1b[91m";
pub const FG_GREEN_B: &'static str = "\x1b[92m";
pub const FG_YELLOW_B: &'static str = "\x1b[93m";
pub const FG_BLUE_B: &'static str = "\x1b[94m";
pub const FG_MAGENTA_B: &'static str = "\x1b[95m";
pub const FG_CYAN_B: &'static str = "\x1b[96m";
pub const FG_WHITE_B: &'static str = "\x1b[97m";

pub const BG_BLACK_B: &'static str = "\x1b[100m";
pub const BG_RED_B: &'static str = "\x1b[101m";
pub const BG_GREEN_B: &'static str = "\x1b[102m";
pub const BG_YELLOW_B: &'static str = "\x1b[103m";
pub const BG_BLUE_B: &'static str = "\x1b[104m";
pub const BG_MAGENTA_B: &'static str = "\x1b[105m";
pub const BG_CYAN_B: &'static str = "\x1b[106m";
pub const BG_WHITE_B: &'static str = "\x1b[107m";
