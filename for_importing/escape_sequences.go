// Maybe this file would be prettier with const ( ... ) and var ( ... ) syntax,
// but it would be hard to implement without fucking up the arrays.

type StringType = string

// Control codes

const BELL StringType = "\x07";

// Cursor control

const CUR_HOME               StringType = "\x1b[H";
const CUR_SET                StringType = "\x1b[%s;%sH";
const CUR_UP                 StringType = "\x1b[%sA";
const CUR_DOWN               StringType = "\x1b[%sB";
const CUR_RIGHT              StringType = "\x1b[%sC";
const CUR_LEFT               StringType = "\x1b[%sD";
const CUR_DOWN_BEG           StringType = "\x1b[%sE";
const CUR_UP_BEG             StringType = "\x1b[%sF";
const CUR_COL                StringType = "\x1b[%sG";

const CUR_UP_ONE             StringType = "\x1b[1A";
const CUR_DOWN_ONE           StringType = "\x1b[1B";
const CUR_RIGHT_ONE          StringType = "\x1b[1C";
const CUR_LEFT_ONE           StringType = "\x1b[1D";

const CUR_COL_HOME           StringType = "\x1b[0G";

const CUR_HIDE               StringType = "\x1b[?25l";
const CUR_SHOW               StringType = "\x1b[?25h";

const CUR_SAVE               StringType = "\x1b7";  // DEC
const CUR_RESTORE            StringType = "\x1b8";  // DEC
const CUR_SAVE_SCO           StringType = "\x1b[s"; // SCO
const CUR_RESTORE_SCO        StringType = "\x1b[u"; // SCO

// Screen operations

const SCREEN_SAVE            StringType = "\x1b[?47h";
const RESTORE_SAVE           StringType = "\x1b[?47l";

const ERASE_SCREEN           StringType = "\x1b[2J";
const ERASE_LINE             StringType = "\x1b[2K";
const ERASE_TO_END           StringType = "\x1b[0J";
const ERASE_TO_LINE_END      StringType = "\x1b[0J";

// Style

const RESET                  StringType = "\x1b[0m";
const BOLD                   StringType = "\x1b[1m";
const DIM                    StringType = "\x1b[2m";
const ITALIC                 StringType = "\x1b[3m";
const UNDERLINE              StringType = "\x1b[4m";
const BLINK                  StringType = "\x1b[5m";
const STRIKE                 StringType = "\x1b[9m";

// 256 colors

const FG_ID                  StringType = "\x1b[38;5;%sm";
const BG_ID                  StringType = "\x1b[48;5;%sm";

// True color

const FG_RGB                 StringType = "\x1b[38;2;%s;%s;%sm";
const BG_RGB                 StringType = "\x1b[48;2;%s;%s;%sm";

// 16 color codes

const FG_BLACK               StringType = "\x1b[30m";
const FG_RED                 StringType = "\x1b[31m";
const FG_GREEN               StringType = "\x1b[32m";
const FG_YELLOW              StringType = "\x1b[33m";
const FG_BLUE                StringType = "\x1b[34m";
const FG_MAGENTA             StringType = "\x1b[35m";
const FG_CYAN                StringType = "\x1b[36m";
const FG_WHITE               StringType = "\x1b[37m";
const FG_DEFAULT             StringType = "\x1b[39m";

const BG_BLACK               StringType = "\x1b[40m";
const BG_RED                 StringType = "\x1b[41m";
const BG_GREEN               StringType = "\x1b[42m";
const BG_YELLOW              StringType = "\x1b[43m";
const BG_BLUE                StringType = "\x1b[44m";
const BG_MAGENTA             StringType = "\x1b[45m";
const BG_CYAN                StringType = "\x1b[46m";
const BG_WHITE               StringType = "\x1b[47m";
const BG_DEFAULT             StringType = "\x1b[49m";

// Bright versions

const FG_BLACK_B             StringType = "\x1b[90m";
const FG_RED_B               StringType = "\x1b[91m";
const FG_GREEN_B             StringType = "\x1b[92m";
const FG_YELLOW_B            StringType = "\x1b[93m";
const FG_BLUE_B              StringType = "\x1b[94m";
const FG_MAGENTA_B           StringType = "\x1b[95m";
const FG_CYAN_B              StringType = "\x1b[96m";
const FG_WHITE_B             StringType = "\x1b[97m";

const BG_BLACK_B             StringType = "\x1b[100m";
const BG_RED_B               StringType = "\x1b[101m";
const BG_GREEN_B             StringType = "\x1b[102m";
const BG_YELLOW_B            StringType = "\x1b[103m";
const BG_BLUE_B              StringType = "\x1b[104m";
const BG_MAGENTA_B           StringType = "\x1b[105m";
const BG_CYAN_B              StringType = "\x1b[106m";
const BG_WHITE_B             StringType = "\x1b[107m";

// Keys

const ESCAPE                 StringType = "\x1b";
const ALT_MOD                StringType = "\x1b";
// Adding escape in front of most key codes gives its alt variant.
// ALT_MOD can be used instead of ESCAPE for clarity.

const TAB                    StringType = "\x09";
const RETURN                 StringType = "\x0d";
const BACKSPACE              StringType = "\x7f";
const SHIFT_TAB              StringType = "\x1b[Z";
const CTRL_BACKSPACE         StringType = "\x08";

var   HOME                   = []StringType{"\x1b[1", "\x1b[H"};
var   END                    = []StringType{"\x1b[4", "\x1b[F"};
const PG_UP                  StringType = "\x1b[5~";
const PG_DOWN                StringType = "\x1b[6~";
const DELETE                 StringType = "\x1b[3~";
const INSERT                 StringType = "\x1b[2~";

// Arrows

const ARROW_UP               StringType = "\x1b[A";
const ARROW_DOWN             StringType = "\x1b[B";
const ARROW_LEFT             StringType = "\x1b[D";
const ARROW_RIGHT            StringType = "\x1b[C";
const SHIFT_ARROW_UP         StringType = "\x1b[1;2A";
const SHIFT_ARROW_DOWN       StringType = "\x1b[1;2B";
const SHIFT_ARROW_LEFT       StringType = "\x1b[1;2D";
const SHIFT_ARROW_RIGHT      StringType = "\x1b[1;2C";
const CTRL_ARROW_UP          StringType = "\x1b[1;5A";
const CTRL_ARROW_DOWN        StringType = "\x1b[1;5B";
const CTRL_ARROW_LEFT        StringType = "\x1b[1;5D";
const CTRL_ARROW_RIGHT       StringType = "\x1b[1;5C";
const CTRL_SHIFT_ARROW_UP    StringType = "\x1b[1;6A";
const CTRL_SHIFT_ARROW_DOWN  StringType = "\x1b[1;6B";
const CTRL_SHIFT_ARROW_LEFT  StringType = "\x1b[1;6D";
const CTRL_SHIFT_ARROW_RIGHT StringType = "\x1b[1;6C";

// Function keys

var   F1                     = []StringType{"\x1bOP", "\x1b[11~"};
var   F2                     = []StringType{"\x1bOQ", "\x1b[12~"};
var   F3                     = []StringType{"\x1bOR", "\x1b[13~"};
var   F4                     = []StringType{"\x1bOS", "\x1b[14~"};
const F5                     StringType = "\x1b[15~";
const F6                     StringType = "\x1b[17~";
const F7                     StringType = "\x1b[18~";
const F8                     StringType = "\x1b[19~";
const F9                     StringType = "\x1b[20~";
const F10                    StringType = "\x1b[21~";
const F11                    StringType = "\x1b[23~";
const F12                    StringType = "\x1b[24~";

// Control + letter

const CTRL_A                 StringType = "\x01";
const CTRL_B                 StringType = "\x02";
const CTRL_C                 StringType = "\x03";
const CTRL_D                 StringType = "\x04";
const CTRL_E                 StringType = "\x05";
const CTRL_F                 StringType = "\x06";
const CTRL_G                 StringType = "\x07";
const CTRL_H                 StringType = "\x08";
const CTRL_I                 StringType = "\x09";
const CTRL_J                 StringType = "\x0a";
const CTRL_K                 StringType = "\x0b";
const CTRL_L                 StringType = "\x0c";
const CTRL_M                 StringType = "\x0d";
const CTRL_N                 StringType = "\x0e";
const CTRL_O                 StringType = "\x0f";
const CTRL_P                 StringType = "\x10";
const CTRL_Q                 StringType = "\x11";
const CTRL_R                 StringType = "\x12";
const CTRL_S                 StringType = "\x13";
const CTRL_T                 StringType = "\x14";
const CTRL_U                 StringType = "\x15";
const CTRL_V                 StringType = "\x16";
const CTRL_W                 StringType = "\x17";
const CTRL_X                 StringType = "\x18";
const CTRL_Y                 StringType = "\x19";
const CTRL_Z                 StringType = "\x1a";
