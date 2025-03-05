#![allow(dead_code)] // disable warnings for unused code

type StringType = &'static str;

// Control codes

pub const BELL:                   StringType = "\x07";

// Cursor control

pub const CUR_HOME:               StringType = "\x1b[H";
pub const CUR_SET:                StringType = "\x1b[{};{}H";
pub const CUR_UP:                 StringType = "\x1b[{}A";
pub const CUR_DOWN:               StringType = "\x1b[{}B";
pub const CUR_RIGHT:              StringType = "\x1b[{}C";
pub const CUR_LEFT:               StringType = "\x1b[{}D";
pub const CUR_DOWN_BEG:           StringType = "\x1b[{}E";
pub const CUR_UP_BEG:             StringType = "\x1b[{}F";
pub const CUR_COL:                StringType = "\x1b[{}G";

pub const CUR_UP_ONE:             StringType = "\x1b[1A";
pub const CUR_DOWN_ONE:           StringType = "\x1b[1B";
pub const CUR_RIGHT_ONE:          StringType = "\x1b[1C";
pub const CUR_LEFT_ONE:           StringType = "\x1b[1D";

pub const CUR_COL_HOME:           StringType = "\x1b[0G";

pub const CUR_HIDE:               StringType = "\x1b[?25l";
pub const CUR_SHOW:               StringType = "\x1b[?25h";

pub const CUR_SAVE_DEC:           StringType = "\x1b "; // DEC
pub const CUR_RESTORE_DEC:        StringType = "\x1b "; // DEC 
pub const CUR_SAVE_SCO:           StringType = "\x1b["; // SCO
pub const CUR_RESTORE_SCO:        StringType = "\x1b["; // SCO

// Screen operations

pub const SCREEN_SAVE:            StringType = "\x1b[?47h";
pub const RESTORE_SAVE:           StringType = "\x1b[?47l";

pub const ERASE_SCREEN:           StringType = "\x1b[2J";
pub const ERASE_LINE:             StringType = "\x1b[2K";
pub const ERASE_TO_END:           StringType = "\x1b[0J";
pub const ERASE_TO_LINE_END:      StringType = "\x1b[0J";

// Style

pub const RESET:                  StringType = "\x1b[0m";
pub const BOLD:                   StringType = "\x1b[1m";
pub const DIM:                    StringType = "\x1b[2m";
pub const ITALIC:                 StringType = "\x1b[3m";
pub const UNDERLINE:              StringType = "\x1b[4m";
pub const BLINK:                  StringType = "\x1b[5m";
pub const STRIKE:                 StringType = "\x1b[9m";

// 256 colors

pub const FG_ID:                  StringType = "\x1b[38;5;{}m";
pub const BG_ID:                  StringType = "\x1b[48;5;{}m";

// True color

pub const FG_RGB:                 StringType = "\x1b[38;2;{};{};{}m";
pub const BG_RGB:                 StringType = "\x1b[48;2;{};{};{}m";

// 16 color codes

pub const FG_BLACK:               StringType = "\x1b[30m";
pub const FG_RED:                 StringType = "\x1b[31m";
pub const FG_GREEN:               StringType = "\x1b[32m";
pub const FG_YELLOW:              StringType = "\x1b[33m";
pub const FG_BLUE:                StringType = "\x1b[34m";
pub const FG_MAGENTA:             StringType = "\x1b[35m";
pub const FG_CYAN:                StringType = "\x1b[36m";
pub const FG_WHITE:               StringType = "\x1b[37m";
pub const FG_DEFAULT:             StringType = "\x1b[39m";

pub const BG_BLACK:               StringType = "\x1b[40m";
pub const BG_RED:                 StringType = "\x1b[41m";
pub const BG_GREEN:               StringType = "\x1b[42m";
pub const BG_YELLOW:              StringType = "\x1b[43m";
pub const BG_BLUE:                StringType = "\x1b[44m";
pub const BG_MAGENTA:             StringType = "\x1b[45m";
pub const BG_CYAN:                StringType = "\x1b[46m";
pub const BG_WHITE:               StringType = "\x1b[47m";
pub const BG_DEFAULT:             StringType = "\x1b[49m";

// Bright versions

pub const FG_BLACK_B:             StringType = "\x1b[90m";
pub const FG_RED_B:               StringType = "\x1b[91m";
pub const FG_GREEN_B:             StringType = "\x1b[92m";
pub const FG_YELLOW_B:            StringType = "\x1b[93m";
pub const FG_BLUE_B:              StringType = "\x1b[94m";
pub const FG_MAGENTA_B:           StringType = "\x1b[95m";
pub const FG_CYAN_B:              StringType = "\x1b[96m";
pub const FG_WHITE_B:             StringType = "\x1b[97m";

pub const BG_BLACK_B:             StringType = "\x1b[100m";
pub const BG_RED_B:               StringType = "\x1b[101m";
pub const BG_GREEN_B:             StringType = "\x1b[102m";
pub const BG_YELLOW_B:            StringType = "\x1b[103m";
pub const BG_BLUE_B:              StringType = "\x1b[104m";
pub const BG_MAGENTA_B:           StringType = "\x1b[105m";
pub const BG_CYAN_B:              StringType = "\x1b[106m";
pub const BG_WHITE_B:             StringType = "\x1b[107m";

// Keys

pub const ESCAPE:                 StringType = "\x1b";
pub const RETURN:                 StringType = "\x0d";
pub const BACKSPACE:              StringType = "\x7f";
pub const TAB:                    StringType = "\x09";
pub const TAB_SHIFT:              StringType = "\x1b[Z";

// Arrows

pub const ARROW_UP:               StringType = "\x1b[A";
pub const ARROW_DOWN:             StringType = "\x1b[B";
pub const ARROW_LEFT:             StringType = "\x1b[D";
pub const ARROW_RIGHT:            StringType = "\x1b[C";
pub const ARROW_UP_SHIFT:         StringType = "\x1b[1;2A";
pub const ARROW_DOWN_SHIFT:       StringType = "\x1b[1;2B";
pub const ARROW_LEFT_SHIFT:       StringType = "\x1b[1;2D";
pub const ARROW_RIGHT_SHIFT:      StringType = "\x1b[1;2C";
pub const ARROW_UP_CTRL:          StringType = "\x1b[1;5A";
pub const ARROW_DOWN_CTRL:        StringType = "\x1b[1;5B";
pub const ARROW_LEFT_CTRL:        StringType = "\x1b[1;5D";
pub const ARROW_RIGHT_CTRL:       StringType = "\x1b[1;5C";
pub const ARROW_UP_CTRL_SHIFT:    StringType = "\x1b[1;6A";
pub const ARROW_DOWN_CTRL_SHIFT:  StringType = "\x1b[1;6B";
pub const ARROW_LEFT_CTRL_SHIFT:  StringType = "\x1b[1;6D";
pub const ARROW_RIGHT_CTRL_SHIFT: StringType = "\x1b[1;6C";
