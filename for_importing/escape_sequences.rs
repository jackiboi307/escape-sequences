#![allow(dead_code)] // disable warnings for unused code

type StringType = &'static str;

// Control codes

pub const BELL:                   StringType = "\007";

// Cursor control

pub const CUR_HOME:               StringType = "\033[H";
pub const CUR_SET:                StringType = "\033[{};{}H";
pub const CUR_UP:                 StringType = "\033[{}A";
pub const CUR_DOWN:               StringType = "\033[{}B";
pub const CUR_RIGHT:              StringType = "\033[{}C";
pub const CUR_LEFT:               StringType = "\033[{}D";
pub const CUR_DOWN_BEG:           StringType = "\033[{}E";
pub const CUR_UP_BEG:             StringType = "\033[{}F";
pub const CUR_COL:                StringType = "\033[{}G";

pub const CUR_UP_ONE:             StringType = "\033[1A";
pub const CUR_DOWN_ONE:           StringType = "\033[1B";
pub const CUR_RIGHT_ONE:          StringType = "\033[1C";
pub const CUR_LEFT_ONE:           StringType = "\033[1D";

pub const CUR_COL_HOME:           StringType = "\033[0G";

pub const CUR_HIDE:               StringType = "\033[?25l";
pub const CUR_SHOW:               StringType = "\033[?25h";

pub const CUR_SAVE_DEC:           StringType = "\033 "; // DEC
pub const CUR_RESTORE_DEC:        StringType = "\033 "; // DEC 
pub const CUR_SAVE_SCO:           StringType = "\033["; // SCO
pub const CUR_RESTORE_SCO:        StringType = "\033["; // SCO

// Screen operations

pub const SCREEN_SAVE:            StringType = "\033[?47h";
pub const RESTORE_SAVE:           StringType = "\033[?47l";

pub const ERASE_SCREEN:           StringType = "\033[2J";
pub const ERASE_LINE:             StringType = "\033[2K";
pub const ERASE_TO_END:           StringType = "\033[0J";
pub const ERASE_TO_LINE_END:      StringType = "\033[0J";

// Style

pub const RESET:                  StringType = "\033[0m";
pub const BOLD:                   StringType = "\033[1m";
pub const DIM:                    StringType = "\033[2m";
pub const ITALIC:                 StringType = "\033[3m";
pub const UNDERLINE:              StringType = "\033[4m";
pub const BLINK:                  StringType = "\033[5m";
pub const STRIKE:                 StringType = "\033[9m";

// 256 colors

pub const FG_ID:                  StringType = "\033[38;5;{}m";
pub const BG_ID:                  StringType = "\033[48;5;{}m";

// True color

pub const FG_RGB:                 StringType = "\033[38;2;{};{};{}m";
pub const BG_RGB:                 StringType = "\033[48;2;{};{};{}m";

// 16 color codes

pub const FG_BLACK:               StringType = "\033[30m";
pub const FG_RED:                 StringType = "\033[31m";
pub const FG_GREEN:               StringType = "\033[32m";
pub const FG_YELLOW:              StringType = "\033[33m";
pub const FG_BLUE:                StringType = "\033[34m";
pub const FG_MAGENTA:             StringType = "\033[35m";
pub const FG_CYAN:                StringType = "\033[36m";
pub const FG_WHITE:               StringType = "\033[37m";
pub const FG_DEFAULT:             StringType = "\033[39m";

pub const BG_BLACK:               StringType = "\033[40m";
pub const BG_RED:                 StringType = "\033[41m";
pub const BG_GREEN:               StringType = "\033[42m";
pub const BG_YELLOW:              StringType = "\033[43m";
pub const BG_BLUE:                StringType = "\033[44m";
pub const BG_MAGENTA:             StringType = "\033[45m";
pub const BG_CYAN:                StringType = "\033[46m";
pub const BG_WHITE:               StringType = "\033[47m";
pub const BG_DEFAULT:             StringType = "\033[49m";

// Bright versions

pub const FG_BLACK_B:             StringType = "\033[90m";
pub const FG_RED_B:               StringType = "\033[91m";
pub const FG_GREEN_B:             StringType = "\033[92m";
pub const FG_YELLOW_B:            StringType = "\033[93m";
pub const FG_BLUE_B:              StringType = "\033[94m";
pub const FG_MAGENTA_B:           StringType = "\033[95m";
pub const FG_CYAN_B:              StringType = "\033[96m";
pub const FG_WHITE_B:             StringType = "\033[97m";

pub const BG_BLACK_B:             StringType = "\033[100m";
pub const BG_RED_B:               StringType = "\033[101m";
pub const BG_GREEN_B:             StringType = "\033[102m";
pub const BG_YELLOW_B:            StringType = "\033[103m";
pub const BG_BLUE_B:              StringType = "\033[104m";
pub const BG_MAGENTA_B:           StringType = "\033[105m";
pub const BG_CYAN_B:              StringType = "\033[106m";
pub const BG_WHITE_B:             StringType = "\033[107m";

// Keys

pub const ESCAPE:                 StringType = "\033";
pub const RETURN:                 StringType = "\r";
pub const BACKSPACE:              StringType = "\x7f";
pub const TAB:                    StringType = "\t";
pub const TAB_SHIFT:              StringType = "\033[Z";

// Arrows

pub const ARROW_UP:               StringType = "\033[A";
pub const ARROW_DOWN:             StringType = "\033[B";
pub const ARROW_LEFT:             StringType = "\033[D";
pub const ARROW_RIGHT:            StringType = "\033[C";
pub const ARROW_UP_SHIFT:         StringType = "\033[1;2A";
pub const ARROW_DOWN_SHIFT:       StringType = "\033[1;2B";
pub const ARROW_LEFT_SHIFT:       StringType = "\033[1;2D";
pub const ARROW_RIGHT_SHIFT:      StringType = "\033[1;2C";
pub const ARROW_UP_CTRL:          StringType = "\033[1;5A";
pub const ARROW_DOWN_CTRL:        StringType = "\033[1;5B";
pub const ARROW_LEFT_CTRL:        StringType = "\033[1;5D";
pub const ARROW_RIGHT_CTRL:       StringType = "\033[1;5C";
pub const ARROW_UP_CTRL_SHIFT:    StringType = "\033[1;6A";
pub const ARROW_DOWN_CTRL_SHIFT:  StringType = "\033[1;6B";
pub const ARROW_LEFT_CTRL_SHIFT:  StringType = "\033[1;6D";
pub const ARROW_RIGHT_CTRL_SHIFT: StringType = "\033[1;6C";
