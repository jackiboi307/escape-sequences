// License: https://unlicense.org/
// Source:  https://github.com/jackiboi307/escape-sequences

package main

// Control codes

const BELL string = "\x07"

// Cursor control

const CUR_HOME string = "\x1b[H"
const CUR_SET string = "\x1b[%s;%sH"
const CUR_UP string = "\x1b[%sA"
const CUR_DOWN string = "\x1b[%sB"
const CUR_RIGHT string = "\x1b[%sC"
const CUR_LEFT string = "\x1b[%sD"
const CUR_DOWN_BEG string = "\x1b[%sE"
const CUR_UP_BEG string = "\x1b[%sF"
const CUR_COL string = "\x1b[%sG"

const CUR_UP_ONE string = "\x1b[1A"
const CUR_DOWN_ONE string = "\x1b[1B"
const CUR_RIGHT_ONE string = "\x1b[1C"
const CUR_LEFT_ONE string = "\x1b[1D"

const CUR_COL_HOME string = "\x1b[0G"

const CUR_HIDE string = "\x1b[?25l"
const CUR_SHOW string = "\x1b[?25h"

const CUR_SAVE string = "\x1b7"
const CUR_RESTORE string = "\x1b8"
const CUR_SAVE_SCO string = "\x1b[s"
const CUR_RESTORE_SCO string = "\x1b[u"

// Screen operations

const SCREEN_SAVE string = "\x1b[?47h"
const RESTORE_SAVE string = "\x1b[?47l"

const ERASE_SCREEN string = "\x1b[2J"
const ERASE_LINE string = "\x1b[2K"
const ERASE_TO_END string = "\x1b[0J"
const ERASE_TO_LINE_END string = "\x1b[0J"

// Style

const RESET string = "\x1b[0m"
const BOLD string = "\x1b[1m"
const DIM string = "\x1b[2m"
const ITALIC string = "\x1b[3m"
const UNDERLINE string = "\x1b[4m"
const BLINK string = "\x1b[5m"
const STRIKE string = "\x1b[9m"

// 256 colors

const FG_ID string = "\x1b[38;5;%sm"
const BG_ID string = "\x1b[48;5;%sm"

// True color

const FG_RGB string = "\x1b[38;2;%s;%s;%sm"
const BG_RGB string = "\x1b[48;2;%s;%s;%sm"

// 16 color codes

const FG_BLACK string = "\x1b[30m"
const FG_RED string = "\x1b[31m"
const FG_GREEN string = "\x1b[32m"
const FG_YELLOW string = "\x1b[33m"
const FG_BLUE string = "\x1b[34m"
const FG_MAGENTA string = "\x1b[35m"
const FG_CYAN string = "\x1b[36m"
const FG_WHITE string = "\x1b[37m"
const FG_DEFAULT string = "\x1b[39m"

const BG_BLACK string = "\x1b[40m"
const BG_RED string = "\x1b[41m"
const BG_GREEN string = "\x1b[42m"
const BG_YELLOW string = "\x1b[43m"
const BG_BLUE string = "\x1b[44m"
const BG_MAGENTA string = "\x1b[45m"
const BG_CYAN string = "\x1b[46m"
const BG_WHITE string = "\x1b[47m"
const BG_DEFAULT string = "\x1b[49m"

// Bright versions

const FG_BLACK_B string = "\x1b[90m"
const FG_RED_B string = "\x1b[91m"
const FG_GREEN_B string = "\x1b[92m"
const FG_YELLOW_B string = "\x1b[93m"
const FG_BLUE_B string = "\x1b[94m"
const FG_MAGENTA_B string = "\x1b[95m"
const FG_CYAN_B string = "\x1b[96m"
const FG_WHITE_B string = "\x1b[97m"

const BG_BLACK_B string = "\x1b[100m"
const BG_RED_B string = "\x1b[101m"
const BG_GREEN_B string = "\x1b[102m"
const BG_YELLOW_B string = "\x1b[103m"
const BG_BLUE_B string = "\x1b[104m"
const BG_MAGENTA_B string = "\x1b[105m"
const BG_CYAN_B string = "\x1b[106m"
const BG_WHITE_B string = "\x1b[107m"
