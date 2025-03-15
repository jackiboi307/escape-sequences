#![allow(dead_code)] // disable warnings for unused code

type StringType = &'static str;

// Control codes

pub const BELL: StringType = "\x07";

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

pub const CUR_SAVE:               StringType = "\x1b7";  // DEC
pub const CUR_RESTORE:            StringType = "\x1b8";  // DEC
pub const CUR_SAVE_SCO:           StringType = "\x1b[s"; // SCO
pub const CUR_RESTORE_SCO:        StringType = "\x1b[u"; // SCO

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
pub const ALT_MOD:                StringType = "\x1b";
// Adding escape in front of most key codes gives its alt variant.
// ALT_MOD can be used instead of ESCAPE for clarity.

pub const TAB:                    StringType = "\x09";
pub const RETURN:                 StringType = "\x0d";
pub const BACKSPACE:              StringType = "\x7f";
pub const SHIFT_TAB:              StringType = "\x1b[Z";
pub const CTRL_BACKSPACE:         StringType = "\x08";

pub const HOME:                   [StringType; 2] = ["\x1b[1", "\x1b[H"];
pub const END:                    [StringType; 2] = ["\x1b[4", "\x1b[F"];
pub const PG_UP:                  StringType = "\x1b[5~";
pub const PG_DOWN:                StringType = "\x1b[6~";
pub const DELETE:                 StringType = "\x1b[3~";
pub const INSERT:                 StringType = "\x1b[2~";

// Arrows

pub const ARROW_UP:               StringType = "\x1b[A";
pub const ARROW_DOWN:             StringType = "\x1b[B";
pub const ARROW_LEFT:             StringType = "\x1b[D";
pub const ARROW_RIGHT:            StringType = "\x1b[C";
pub const SHIFT_ARROW_UP:         StringType = "\x1b[1;2A";
pub const SHIFT_ARROW_DOWN:       StringType = "\x1b[1;2B";
pub const SHIFT_ARROW_LEFT:       StringType = "\x1b[1;2D";
pub const SHIFT_ARROW_RIGHT:      StringType = "\x1b[1;2C";
pub const CTRL_ARROW_UP:          StringType = "\x1b[1;5A";
pub const CTRL_ARROW_DOWN:        StringType = "\x1b[1;5B";
pub const CTRL_ARROW_LEFT:        StringType = "\x1b[1;5D";
pub const CTRL_ARROW_RIGHT:       StringType = "\x1b[1;5C";
pub const CTRL_SHIFT_ARROW_UP:    StringType = "\x1b[1;6A";
pub const CTRL_SHIFT_ARROW_DOWN:  StringType = "\x1b[1;6B";
pub const CTRL_SHIFT_ARROW_LEFT:  StringType = "\x1b[1;6D";
pub const CTRL_SHIFT_ARROW_RIGHT: StringType = "\x1b[1;6C";

// Function keys

pub const F1:                     [StringType; 2] = ["\x1bOP", "\x1b[11~"];
pub const F2:                     [StringType; 2] = ["\x1bOQ", "\x1b[12~"];
pub const F3:                     [StringType; 2] = ["\x1bOR", "\x1b[13~"];
pub const F4:                     [StringType; 2] = ["\x1bOS", "\x1b[14~"];
pub const F5:                     StringType = "\x1b[15~";
pub const F6:                     StringType = "\x1b[17~";
pub const F7:                     StringType = "\x1b[18~";
pub const F8:                     StringType = "\x1b[19~";
pub const F9:                     StringType = "\x1b[20~";
pub const F10:                    StringType = "\x1b[21~";
pub const F11:                    StringType = "\x1b[23~";
pub const F12:                    StringType = "\x1b[24~";

// Control + letter

pub const CTRL_A:                 StringType = "\x01";
pub const CTRL_B:                 StringType = "\x02";
pub const CTRL_C:                 StringType = "\x03";
pub const CTRL_D:                 StringType = "\x04";
pub const CTRL_E:                 StringType = "\x05";
pub const CTRL_F:                 StringType = "\x06";
pub const CTRL_G:                 StringType = "\x07";
pub const CTRL_H:                 StringType = "\x08";
pub const CTRL_I:                 StringType = "\x09";
pub const CTRL_J:                 StringType = "\x0a";
pub const CTRL_K:                 StringType = "\x0b";
pub const CTRL_L:                 StringType = "\x0c";
pub const CTRL_M:                 StringType = "\x0d";
pub const CTRL_N:                 StringType = "\x0e";
pub const CTRL_O:                 StringType = "\x0f";
pub const CTRL_P:                 StringType = "\x10";
pub const CTRL_Q:                 StringType = "\x11";
pub const CTRL_R:                 StringType = "\x12";
pub const CTRL_S:                 StringType = "\x13";
pub const CTRL_T:                 StringType = "\x14";
pub const CTRL_U:                 StringType = "\x15";
pub const CTRL_V:                 StringType = "\x16";
pub const CTRL_W:                 StringType = "\x17";
pub const CTRL_X:                 StringType = "\x18";
pub const CTRL_Y:                 StringType = "\x19";
pub const CTRL_Z:                 StringType = "\x1a";
