// Control codes

const BELL: &'static str = "\x07";

// Cursor control

const CUR_HOME: &'static str = "\x1b[H";
const CUR_SET: &'static str = "\x1b[{};{}H";
const CUR_UP: &'static str = "\x1b[{}A";
const CUR_DOWN: &'static str = "\x1b[{}B";
const CUR_RIGHT: &'static str = "\x1b[{}C";
const CUR_LEFT: &'static str = "\x1b[{}D";
const CUR_DOWN_BEG: &'static str = "\x1b[{}E";
const CUR_UP_BEG: &'static str = "\x1b[{}F";
const CUR_COL: &'static str = "\x1b[{}G";

const CUR_UP_ONE: &'static str = "\x1b[1A";
const CUR_DOWN_ONE: &'static str = "\x1b[1B";
const CUR_RIGHT_ONE: &'static str = "\x1b[1C";
const CUR_LEFT_ONE: &'static str = "\x1b[1D";

const CUR_COL_HOME: &'static str = "\x1b[0G";

const CUR_HIDE: &'static str = "\x1b[?25l";
const CUR_SHOW: &'static str = "\x1b[?25h";

const CUR_SAVE: &'static str = "\x1b7'";
const CUR_RESTORE: &'static str = "\x1b8'";
const CUR_SAVE_SCO: &'static str = "\x1b[s";
const CUR_RESTORE_SCO: &'static str = "\x1b[u";

// Screen operations

const SCREEN_SAVE: &'static str = "\x1b[?47h";
const RESTORE_SAVE: &'static str = "\x1b[?47l";

const ERASE_SCREEN: &'static str = "\x1b[2J";
const ERASE_LINE: &'static str = "\x1b[2K";
const ERASE_TO_END: &'static str = "\x1b[0J";
const ERASE_TO_LINE_END: &'static str = "\x1b[0J";

// Style

const RESET: &'static str = "\x1b[0m";
const BOLD: &'static str = "\x1b[1m";
const DIM: &'static str = "\x1b[2m";
const ITALIC: &'static str = "\x1b[3m";
const UNDERLINE: &'static str = "\x1b[4m";
const BLINK: &'static str = "\x1b[5m";
const STRIKE: &'static str = "\x1b[9m";

// 256 colors

const FG_ID: &'static str = "\x1b[38;5;{}m";
const BG_ID: &'static str = "\x1b[48;5;{}m";

// True color

const FG_RGB: &'static str = "\x1b[38;2;{};{};{}m";
const BG_RGB: &'static str = "\x1b[48;2;{};{};{}m";

// 16 color codes

const FG_BLACK: &'static str = "\x1b[30m";
const FG_RED: &'static str = "\x1b[31m";
const FG_GREEN: &'static str = "\x1b[32m";
const FG_YELLOW: &'static str = "\x1b[33m";
const FG_BLUE: &'static str = "\x1b[34m";
const FG_MAGENTA: &'static str = "\x1b[35m";
const FG_CYAN: &'static str = "\x1b[36m";
const FG_WHITE: &'static str = "\x1b[37m";
const FG_DEFAULT: &'static str = "\x1b[39m";

const BG_BLACK: &'static str = "\x1b[40m";
const BG_RED: &'static str = "\x1b[41m";
const BG_GREEN: &'static str = "\x1b[42m";
const BG_YELLOW: &'static str = "\x1b[43m";
const BG_BLUE: &'static str = "\x1b[44m";
const BG_MAGENTA: &'static str = "\x1b[45m";
const BG_CYAN: &'static str = "\x1b[46m";
const BG_WHITE: &'static str = "\x1b[47m";
const BG_DEFAULT: &'static str = "\x1b[49m";

// Bright versions

const FG_BLACK_B: &'static str = "\x1b[90m";
const FG_RED_B: &'static str = "\x1b[91m";
const FG_GREEN_B: &'static str = "\x1b[92m";
const FG_YELLOW_B: &'static str = "\x1b[93m";
const FG_BLUE_B: &'static str = "\x1b[94m";
const FG_MAGENTA_B: &'static str = "\x1b[95m";
const FG_CYAN_B: &'static str = "\x1b[96m";
const FG_WHITE_B: &'static str = "\x1b[97m";

const BG_BLACK_B: &'static str = "\x1b[100m";
const BG_RED_B: &'static str = "\x1b[101m";
const BG_GREEN_B: &'static str = "\x1b[102m";
const BG_YELLOW_B: &'static str = "\x1b[103m";
const BG_BLUE_B: &'static str = "\x1b[104m";
const BG_MAGENTA_B: &'static str = "\x1b[105m";
const BG_CYAN_B: &'static str = "\x1b[106m";
const BG_WHITE_B: &'static str = "\x1b[107m";

// Keys

const ESCAPE: &'static str = "\x1b";
const ALT_MOD: &'static str = "\x1b";
// Adding escape in front of most key codes gives its alt variant.
// ALT_MOD can be used instead of ESCAPE for clarity.

const TAB: &'static str = "\x09";
const RETURN: &'static str = "\x0d";
const BACKSPACE: &'static str = "\x7f";
const SHIFT_TAB: &'static str = "\x1b[Z";
const CTRL_BACKSPACE: &'static str = "\x08";

const HOME: &'static str = "\x1b[H";
const END: &'static str = "\x1b[F";
const PG_UP: &'static str = "\x1b[5~";
const PG_DOWN: &'static str = "\x1b[6~";
const DELETE: &'static str = "\x1b[3~";
const INSERT: &'static str = "\x1b[2~";

const CTRL_HOME: &'static str = "\x1b[1;5H";
const CTRL_END: &'static str = "\x1b[1;5F";
const CTRL_PG_UP: &'static str = "\x1b[5;5~";
const CTRL_PG_DOWN: &'static str = "\x1b[6;5~";
const CTRL_DELETE: &'static str = "\x1b[3;5~";
const CTRL_INSERT: &'static str = "\x1b[2;5~";

// Arrows

const ARROW_UP: &'static str = "\x1b[A";
const ARROW_DOWN: &'static str = "\x1b[B";
const ARROW_LEFT: &'static str = "\x1b[D";
const ARROW_RIGHT: &'static str = "\x1b[C";
const SHIFT_ARROW_UP: &'static str = "\x1b[1;2A";
const SHIFT_ARROW_DOWN: &'static str = "\x1b[1;2B";
const SHIFT_ARROW_LEFT: &'static str = "\x1b[1;2D";
const SHIFT_ARROW_RIGHT: &'static str = "\x1b[1;2C";
const CTRL_ARROW_UP: &'static str = "\x1b[1;5A";
const CTRL_ARROW_DOWN: &'static str = "\x1b[1;5B";
const CTRL_ARROW_LEFT: &'static str = "\x1b[1;5D";
const CTRL_ARROW_RIGHT: &'static str = "\x1b[1;5C";
const CTRL_SHIFT_ARROW_UP: &'static str = "\x1b[1;6A";
const CTRL_SHIFT_ARROW_DOWN: &'static str = "\x1b[1;6B";
const CTRL_SHIFT_ARROW_LEFT: &'static str = "\x1b[1;6D";
const CTRL_SHIFT_ARROW_RIGHT: &'static str = "\x1b[1;6C";

// Function keys

const F1: &'static str = "\x1bOP";
const F2: &'static str = "\x1bOQ";
const F3: &'static str = "\x1bOR";
const F4: &'static str = "\x1bOS";
const F5: &'static str = "\x1b[15~";
const F6: &'static str = "\x1b[17~";
const F7: &'static str = "\x1b[18~";
const F8: &'static str = "\x1b[19~";
const F9: &'static str = "\x1b[20~";
const F10: &'static str = "\x1b[21~";
const F11: &'static str = "\x1b[23~";
const F12: &'static str = "\x1b[24~";

// Control + letter

const CTRL_A: &'static str = "\x01";
const CTRL_B: &'static str = "\x02";
const CTRL_C: &'static str = "\x03";
const CTRL_D: &'static str = "\x04";
const CTRL_E: &'static str = "\x05";
const CTRL_F: &'static str = "\x06";
const CTRL_G: &'static str = "\x07";
const CTRL_H: &'static str = "\x08";
const CTRL_I: &'static str = "\x09";
const CTRL_J: &'static str = "\x0a";
const CTRL_K: &'static str = "\x0b";
const CTRL_L: &'static str = "\x0c";
const CTRL_M: &'static str = "\x0d";
const CTRL_N: &'static str = "\x0e";
const CTRL_O: &'static str = "\x0f";
const CTRL_P: &'static str = "\x10";
const CTRL_Q: &'static str = "\x11";
const CTRL_R: &'static str = "\x12";
const CTRL_S: &'static str = "\x13";
const CTRL_T: &'static str = "\x14";
const CTRL_U: &'static str = "\x15";
const CTRL_V: &'static str = "\x16";
const CTRL_W: &'static str = "\x17";
const CTRL_X: &'static str = "\x18";
const CTRL_Y: &'static str = "\x19";
const CTRL_Z: &'static str = "\x1a";
