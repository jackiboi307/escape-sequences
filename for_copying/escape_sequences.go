// Control codes

const BELL string = "\x07";

// Cursor control

const CUR_HOME string = "\x1b[H";
const CUR_SET string = "\x1b[%s;%sH";
const CUR_UP string = "\x1b[%sA";
const CUR_DOWN string = "\x1b[%sB";
const CUR_RIGHT string = "\x1b[%sC";
const CUR_LEFT string = "\x1b[%sD";
const CUR_DOWN_BEG string = "\x1b[%sE";
const CUR_UP_BEG string = "\x1b[%sF";
const CUR_COL string = "\x1b[%sG";

const CUR_UP_ONE string = "\x1b[1A";
const CUR_DOWN_ONE string = "\x1b[1B";
const CUR_RIGHT_ONE string = "\x1b[1C";
const CUR_LEFT_ONE string = "\x1b[1D";

const CUR_COL_HOME string = "\x1b[0G";

const CUR_HIDE string = "\x1b[?25l";
const CUR_SHOW string = "\x1b[?25h";

const CUR_SAVE string = "\x1b7";
const CUR_RESTORE string = "\x1b8";
const CUR_SAVE_SCO string = "\x1b[s";
const CUR_RESTORE_SCO string = "\x1b[u";

// Screen operations

const SCREEN_SAVE string = "\x1b[?47h";
const RESTORE_SAVE string = "\x1b[?47l";

const ERASE_SCREEN string = "\x1b[2J";
const ERASE_LINE string = "\x1b[2K";
const ERASE_TO_END string = "\x1b[0J";
const ERASE_TO_LINE_END string = "\x1b[0J";

// Style

const RESET string = "\x1b[0m";
const BOLD string = "\x1b[1m";
const DIM string = "\x1b[2m";
const ITALIC string = "\x1b[3m";
const UNDERLINE string = "\x1b[4m";
const BLINK string = "\x1b[5m";
const STRIKE string = "\x1b[9m";

// 256 colors

const FG_ID string = "\x1b[38;5;%sm";
const BG_ID string = "\x1b[48;5;%sm";

// True color

const FG_RGB string = "\x1b[38;2;%s;%s;%sm";
const BG_RGB string = "\x1b[48;2;%s;%s;%sm";

// 16 color codes

const FG_BLACK string = "\x1b[30m";
const FG_RED string = "\x1b[31m";
const FG_GREEN string = "\x1b[32m";
const FG_YELLOW string = "\x1b[33m";
const FG_BLUE string = "\x1b[34m";
const FG_MAGENTA string = "\x1b[35m";
const FG_CYAN string = "\x1b[36m";
const FG_WHITE string = "\x1b[37m";
const FG_DEFAULT string = "\x1b[39m";

const BG_BLACK string = "\x1b[40m";
const BG_RED string = "\x1b[41m";
const BG_GREEN string = "\x1b[42m";
const BG_YELLOW string = "\x1b[43m";
const BG_BLUE string = "\x1b[44m";
const BG_MAGENTA string = "\x1b[45m";
const BG_CYAN string = "\x1b[46m";
const BG_WHITE string = "\x1b[47m";
const BG_DEFAULT string = "\x1b[49m";

// Bright versions

const FG_BLACK_B string = "\x1b[90m";
const FG_RED_B string = "\x1b[91m";
const FG_GREEN_B string = "\x1b[92m";
const FG_YELLOW_B string = "\x1b[93m";
const FG_BLUE_B string = "\x1b[94m";
const FG_MAGENTA_B string = "\x1b[95m";
const FG_CYAN_B string = "\x1b[96m";
const FG_WHITE_B string = "\x1b[97m";

const BG_BLACK_B string = "\x1b[100m";
const BG_RED_B string = "\x1b[101m";
const BG_GREEN_B string = "\x1b[102m";
const BG_YELLOW_B string = "\x1b[103m";
const BG_BLUE_B string = "\x1b[104m";
const BG_MAGENTA_B string = "\x1b[105m";
const BG_CYAN_B string = "\x1b[106m";
const BG_WHITE_B string = "\x1b[107m";

// Keys

const ESCAPE string = "\x1b";
const ALT_MOD string = "\x1b";
// Adding escape in front of most key codes gives its alt variant.
// ALT_MOD can be used instead of ESCAPE for clarity.

const TAB string = "\x09";
const RETURN string = "\x0d";
const BACKSPACE string = "\x7f";
const SHIFT_TAB string = "\x1b[Z";
const CTRL_BACKSPACE string = "\x08";

var HOME = []string{"\x1b[1", "\x1b[H"};
var END = []string{"\x1b[4", "\x1b[F"};
const PG_UP string = "\x1b[5~";
const PG_DOWN string = "\x1b[6~";
const DELETE string = "\x1b[3~";
const INSERT string = "\x1b[2~";

// Arrows

const ARROW_UP string = "\x1b[A";
const ARROW_DOWN string = "\x1b[B";
const ARROW_LEFT string = "\x1b[D";
const ARROW_RIGHT string = "\x1b[C";
const SHIFT_ARROW_UP string = "\x1b[1;2A";
const SHIFT_ARROW_DOWN string = "\x1b[1;2B";
const SHIFT_ARROW_LEFT string = "\x1b[1;2D";
const SHIFT_ARROW_RIGHT string = "\x1b[1;2C";
const CTRL_ARROW_UP string = "\x1b[1;5A";
const CTRL_ARROW_DOWN string = "\x1b[1;5B";
const CTRL_ARROW_LEFT string = "\x1b[1;5D";
const CTRL_ARROW_RIGHT string = "\x1b[1;5C";
const CTRL_SHIFT_ARROW_UP string = "\x1b[1;6A";
const CTRL_SHIFT_ARROW_DOWN string = "\x1b[1;6B";
const CTRL_SHIFT_ARROW_LEFT string = "\x1b[1;6D";
const CTRL_SHIFT_ARROW_RIGHT string = "\x1b[1;6C";

// Function keys

var F1 = []string{"\x1bOP", "\x1b[11~"};
var F2 = []string{"\x1bOQ", "\x1b[12~"};
var F3 = []string{"\x1bOR", "\x1b[13~"};
var F4 = []string{"\x1bOS", "\x1b[14~"};
const F5 string = "\x1b[15~";
const F6 string = "\x1b[17~";
const F7 string = "\x1b[18~";
const F8 string = "\x1b[19~";
const F9 string = "\x1b[20~";
const F10 string = "\x1b[21~";
const F11 string = "\x1b[23~";
const F12 string = "\x1b[24~";

// Control + letter

const CTRL_A string = "\x01";
const CTRL_B string = "\x02";
const CTRL_C string = "\x03";
const CTRL_D string = "\x04";
const CTRL_E string = "\x05";
const CTRL_F string = "\x06";
const CTRL_G string = "\x07";
const CTRL_H string = "\x08";
const CTRL_I string = "\x09";
const CTRL_J string = "\x0a";
const CTRL_K string = "\x0b";
const CTRL_L string = "\x0c";
const CTRL_M string = "\x0d";
const CTRL_N string = "\x0e";
const CTRL_O string = "\x0f";
const CTRL_P string = "\x10";
const CTRL_Q string = "\x11";
const CTRL_R string = "\x12";
const CTRL_S string = "\x13";
const CTRL_T string = "\x14";
const CTRL_U string = "\x15";
const CTRL_V string = "\x16";
const CTRL_W string = "\x17";
const CTRL_X string = "\x18";
const CTRL_Y string = "\x19";
const CTRL_Z string = "\x1a";
