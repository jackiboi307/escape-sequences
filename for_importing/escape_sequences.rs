#![allow(...)]

type string = &'static str;

// Control codes

pub const BELL:                   string = "\007";

// Cursor control

pub const CUR_HOME:               string = "\033[H";
pub const CUR_SET:                string = "\033[{};{}H";
pub const CUR_UP:                 string = "\033[{}A";
pub const CUR_DOWN:               string = "\033[{}B";
pub const CUR_RIGHT:              string = "\033[{}C";
pub const CUR_LEFT:               string = "\033[{}D";
pub const CUR_DOWN_BEG:           string = "\033[{}E";
pub const CUR_UP_BEG:             string = "\033[{}F";
pub const CUR_COL:                string = "\033[{}G";

pub const CUR_UP_ONE:             string = "\033[1A";
pub const CUR_DOWN_ONE:           string = "\033[1B";
pub const CUR_RIGHT_ONE:          string = "\033[1C";
pub const CUR_LEFT_ONE:           string = "\033[1D";

pub const CUR_COL_HOME:           string = "\033[0G";

pub const CUR_HIDE:               string = "\033[?25l";
pub const CUR_SHOW:               string = "\033[?25h";

pub const CUR_SAVE_DEC:           string = "\033 "; // DEC
pub const CUR_RESTORE_DEC:        string = "\033 "; // DEC 
pub const CUR_SAVE_SCO:           string = "\033["; // SCO
pub const CUR_RESTORE_SCO:        string = "\033["; // SCO

// Screen operations

pub const SCREEN_SAVE:            string = "\033[?47h";
pub const RESTORE_SAVE:           string = "\033[?47l";

pub const ERASE_SCREEN:           string = "\033[2J";
pub const ERASE_LINE:             string = "\033[2K";
pub const ERASE_TO_END:           string = "\033[0J";
pub const ERASE_TO_LINE_END:      string = "\033[0J";

// Style

pub const RESET:                  string = "\033[0m";
pub const BOLD:                   string = "\033[1m";
pub const DIM:                    string = "\033[2m";
pub const ITALIC:                 string = "\033[3m";
pub const UNDERLINE:              string = "\033[4m";
pub const BLINK:                  string = "\033[5m";
pub const STRIKE:                 string = "\033[9m";

// 256 colors

pub const FG_ID:                  string = "\033[38;5;{}m";
pub const BG_ID:                  string = "\033[48;5;{}m";

// True color

pub const FG_RGB:                 string = "\033[38;2;{};{};{}m";
pub const BG_RGB:                 string = "\033[48;2;{};{};{}m";

// 16 color codes

pub const FG_BLACK:               string = "\033[30m";
pub const FG_RED:                 string = "\033[31m";
pub const FG_GREEN:               string = "\033[32m";
pub const FG_YELLOW:              string = "\033[33m";
pub const FG_BLUE:                string = "\033[34m";
pub const FG_MAGENTA:             string = "\033[35m";
pub const FG_CYAN:                string = "\033[36m";
pub const FG_WHITE:               string = "\033[37m";
pub const FG_DEFAULT:             string = "\033[39m";

pub const BG_BLACK:               string = "\033[40m";
pub const BG_RED:                 string = "\033[41m";
pub const BG_GREEN:               string = "\033[42m";
pub const BG_YELLOW:              string = "\033[43m";
pub const BG_BLUE:                string = "\033[44m";
pub const BG_MAGENTA:             string = "\033[45m";
pub const BG_CYAN:                string = "\033[46m";
pub const BG_WHITE:               string = "\033[47m";
pub const BG_DEFAULT:             string = "\033[49m";

// Bright versions

pub const FG_BLACK_B:             string = "\033[90m";
pub const FG_RED_B:               string = "\033[91m";
pub const FG_GREEN_B:             string = "\033[92m";
pub const FG_YELLOW_B:            string = "\033[93m";
pub const FG_BLUE_B:              string = "\033[94m";
pub const FG_MAGENTA_B:           string = "\033[95m";
pub const FG_CYAN_B:              string = "\033[96m";
pub const FG_WHITE_B:             string = "\033[97m";

pub const BG_BLACK_B:             string = "\033[100m";
pub const BG_RED_B:               string = "\033[101m";
pub const BG_GREEN_B:             string = "\033[102m";
pub const BG_YELLOW_B:            string = "\033[103m";
pub const BG_BLUE_B:              string = "\033[104m";
pub const BG_MAGENTA_B:           string = "\033[105m";
pub const BG_CYAN_B:              string = "\033[106m";
pub const BG_WHITE_B:             string = "\033[107m";

// Keys

pub const ESCAPE:                 string = "\033";
pub const RETURN:                 string = "\r";
pub const BACKSPACE:              string = "\x7f";
pub const TAB:                    string = "\t";
pub const TAB_SHIFT:              string = "\033[Z";

// Arrows

pub const ARROW_UP:               string = "\033[A";
pub const ARROW_DOWN:             string = "\033[B";
pub const ARROW_LEFT:             string = "\033[D";
pub const ARROW_RIGHT:            string = "\033[C";
pub const ARROW_UP_SHIFT:         string = "\033[1;2A";
pub const ARROW_DOWN_SHIFT:       string = "\033[1;2B";
pub const ARROW_LEFT_SHIFT:       string = "\033[1;2D";
pub const ARROW_RIGHT_SHIFT:      string = "\033[1;2C";
pub const ARROW_UP_CTRL:          string = "\033[1;5A";
pub const ARROW_DOWN_CTRL:        string = "\033[1;5B";
pub const ARROW_LEFT_CTRL:        string = "\033[1;5D";
pub const ARROW_RIGHT_CTRL:       string = "\033[1;5C";
pub const ARROW_UP_CTRL_SHIFT:    string = "\033[1;6A";
pub const ARROW_DOWN_CTRL_SHIFT:  string = "\033[1;6B";
pub const ARROW_LEFT_CTRL_SHIFT:  string = "\033[1;6D";
pub const ARROW_RIGHT_CTRL_SHIFT: string = "\033[1;6C";
