// Control codes

const BELL string = "\007"

// Cursor control

const CUR_HOME string = "\033[H"
const CUR_SET string = "\033[%s;%sH"
const CUR_UP string = "\033[%sA"
const CUR_DOWN string = "\033[%sB"
const CUR_RIGHT string = "\033[%sC"
const CUR_LEFT string = "\033[%sD"
const CUR_DOWN_BEG string = "\033[%sE"
const CUR_UP_BEG string = "\033[%sF"
const CUR_COL string = "\033[%sG"

const CUR_UP_ONE string = "\033[1A"
const CUR_DOWN_ONE string = "\033[1B"
const CUR_RIGHT_ONE string = "\033[1C"
const CUR_LEFT_ONE string = "\033[1D"

const CUR_COL_HOME string = "\033[0G"

const CUR_HIDE string = "\033[?25l"
const CUR_SHOW string = "\033[?25h"

const CUR_SAVE_DEC string = "\033 " // DEC
const CUR_RESTORE_DEC string = "\033 " // DEC 
const CUR_SAVE_SCO string = "\033[" // SCO
const CUR_RESTORE_SCO string = "\033[" // SCO

// Screen operations

const SCREEN_SAVE string = "\033[?47h"
const RESTORE_SAVE string = "\033[?47l"

const ERASE_SCREEN string = "\033[2J"
const ERASE_LINE string = "\033[2K"
const ERASE_TO_END string = "\033[0J"
const ERASE_TO_LINE_END string = "\033[0J"

// Style

const RESET string = "\033[0m"
const BOLD string = "\033[1m"
const DIM string = "\033[2m"
const ITALIC string = "\033[3m"
const UNDERLINE string = "\033[4m"
const BLINK string = "\033[5m"
const STRIKE string = "\033[9m"

// 256 colors

const FG_ID string = "\033[38;5;%sm"
const BG_ID string = "\033[48;5;%sm"

// True color

const FG_RGB string = "\033[38;2;%s;%s;%sm"
const BG_RGB string = "\033[48;2;%s;%s;%sm"

// 16 color codes

const FG_BLACK string = "\033[30m"
const FG_RED string = "\033[31m"
const FG_GREEN string = "\033[32m"
const FG_YELLOW string = "\033[33m"
const FG_BLUE string = "\033[34m"
const FG_MAGENTA string = "\033[35m"
const FG_CYAN string = "\033[36m"
const FG_WHITE string = "\033[37m"
const FG_DEFAULT string = "\033[39m"

const BG_BLACK string = "\033[40m"
const BG_RED string = "\033[41m"
const BG_GREEN string = "\033[42m"
const BG_YELLOW string = "\033[43m"
const BG_BLUE string = "\033[44m"
const BG_MAGENTA string = "\033[45m"
const BG_CYAN string = "\033[46m"
const BG_WHITE string = "\033[47m"
const BG_DEFAULT string = "\033[49m"

// Bright versions

const FG_BLACK_B string = "\033[90m"
const FG_RED_B string = "\033[91m"
const FG_GREEN_B string = "\033[92m"
const FG_YELLOW_B string = "\033[93m"
const FG_BLUE_B string = "\033[94m"
const FG_MAGENTA_B string = "\033[95m"
const FG_CYAN_B string = "\033[96m"
const FG_WHITE_B string = "\033[97m"

const BG_BLACK_B string = "\033[100m"
const BG_RED_B string = "\033[101m"
const BG_GREEN_B string = "\033[102m"
const BG_YELLOW_B string = "\033[103m"
const BG_BLUE_B string = "\033[104m"
const BG_MAGENTA_B string = "\033[105m"
const BG_CYAN_B string = "\033[106m"
const BG_WHITE_B string = "\033[107m"

// Keys

const ESCAPE string = "\033"
const RETURN string = "\r"
const BACKSPACE string = "\x7f"
const TAB string = "\t"
const TAB_SHIFT string = "\033[Z"

// Arrows

const ARROW_UP string = "\033[A"
const ARROW_DOWN string = "\033[B"
const ARROW_LEFT string = "\033[D"
const ARROW_RIGHT string = "\033[C"
const ARROW_UP_SHIFT string = "\033[1;2A"
const ARROW_DOWN_SHIFT string = "\033[1;2B"
const ARROW_LEFT_SHIFT string = "\033[1;2D"
const ARROW_RIGHT_SHIFT string = "\033[1;2C"
const ARROW_UP_CTRL string = "\033[1;5A"
const ARROW_DOWN_CTRL string = "\033[1;5B"
const ARROW_LEFT_CTRL string = "\033[1;5D"
const ARROW_RIGHT_CTRL string = "\033[1;5C"
const ARROW_UP_CTRL_SHIFT string = "\033[1;6A"
const ARROW_DOWN_CTRL_SHIFT string = "\033[1;6B"
const ARROW_LEFT_CTRL_SHIFT string = "\033[1;6D"
const ARROW_RIGHT_CTRL_SHIFT string = "\033[1;6C"
