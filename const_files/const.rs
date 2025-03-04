#![allow(...)]

// Control codes

const BELL:                   &'static str = "'\007'";

// Cursor control

const CUR_HOME:               &'static str = "'\033[H'";
const CUR_SET:                &'static str = "'\033[{};{}H'";
const CUR_UP:                 &'static str = "'\033[{}A'";
const CUR_DOWN:               &'static str = "'\033[{}B'";
const CUR_RIGHT:              &'static str = "'\033[{}C'";
const CUR_LEFT:               &'static str = "'\033[{}D'";
const CUR_DOWN_BEG:           &'static str = "'\033[{}E'";
const CUR_UP_BEG:             &'static str = "'\033[{}F'";
const CUR_COL:                &'static str = "'\033[{}G'";

const CUR_UP_ONE:             &'static str = "'\033[1A'";
const CUR_DOWN_ONE:           &'static str = "'\033[1B'";
const CUR_RIGHT_ONE:          &'static str = "'\033[1C'";
const CUR_LEFT_ONE:           &'static str = "'\033[1D'";

const CUR_COL_HOME:           &'static str = "'\033[0G'";

const CUR_HIDE:               &'static str = "'\033[?25l'";
const CUR_SHOW:               &'static str = "'\033[?25h'";

const CUR_SAVE_DEC:           &'static str = "'\033 7"; // DEC
const CUR_RESTORE_DEC:        &'static str = "'\033 8"; // DEC 
const CUR_SAVE_SCO:           &'static str = "'\033[s"; // SCO
const CUR_RESTORE_SCO:        &'static str = "'\033[u"; // SCO

// Screen operations

const SCREEN_SAVE:            &'static str = "'\033[?47h'";
const RESTORE_SAVE:           &'static str = "'\033[?47l'";

const ERASE_SCREEN:           &'static str = "'\033[2J'";
const ERASE_LINE:             &'static str = "'\033[2K'";
const ERASE_TO_END:           &'static str = "'\033[0J'";
const ERASE_TO_LINE_END:      &'static str = "'\033[0J'";

// Style

const RESET:                  &'static str = "'\033[0m'";
const BOLD:                   &'static str = "'\033[1m'";
const DIM:                    &'static str = "'\033[2m'";
const ITALIC:                 &'static str = "'\033[3m'";
const UNDERLINE:              &'static str = "'\033[4m'";
const BLINK:                  &'static str = "'\033[5m'";
const STRIKE:                 &'static str = "'\033[9m'";

// 256 colors

const FG_ID:                  &'static str = "'\033[38;5;{}m'";
const BG_ID:                  &'static str = "'\033[48;5;{}m'";

// True color

const FG_RGB:                 &'static str = "'\033[38;2;{};{};{}m'";
const BG_RGB:                 &'static str = "'\033[48;2;{};{};{}m'";

// 16 color codes

const FG_BLACK:               &'static str = "'\033[30m'";
const FG_RED:                 &'static str = "'\033[31m'";
const FG_GREEN:               &'static str = "'\033[32m'";
const FG_YELLOW:              &'static str = "'\033[33m'";
const FG_BLUE:                &'static str = "'\033[34m'";
const FG_MAGENTA:             &'static str = "'\033[35m'";
const FG_CYAN:                &'static str = "'\033[36m'";
const FG_WHITE:               &'static str = "'\033[37m'";
const FG_DEFAULT:             &'static str = "'\033[39m'";

const BG_BLACK:               &'static str = "'\033[40m'";
const BG_RED:                 &'static str = "'\033[41m'";
const BG_GREEN:               &'static str = "'\033[42m'";
const BG_YELLOW:              &'static str = "'\033[43m'";
const BG_BLUE:                &'static str = "'\033[44m'";
const BG_MAGENTA:             &'static str = "'\033[45m'";
const BG_CYAN:                &'static str = "'\033[46m'";
const BG_WHITE:               &'static str = "'\033[47m'";
const BG_DEFAULT:             &'static str = "'\033[49m'";

// Bright versions

const FG_BLACK_B:             &'static str = "'\033[90m'";
const FG_RED_B:               &'static str = "'\033[91m'";
const FG_GREEN_B:             &'static str = "'\033[92m'";
const FG_YELLOW_B:            &'static str = "'\033[93m'";
const FG_BLUE_B:              &'static str = "'\033[94m'";
const FG_MAGENTA_B:           &'static str = "'\033[95m'";
const FG_CYAN_B:              &'static str = "'\033[96m'";
const FG_WHITE_B:             &'static str = "'\033[97m'";

const BG_BLACK_B:             &'static str = "'\033[100m'";
const BG_RED_B:               &'static str = "'\033[101m'";
const BG_GREEN_B:             &'static str = "'\033[102m'";
const BG_YELLOW_B:            &'static str = "'\033[103m'";
const BG_BLUE_B:              &'static str = "'\033[104m'";
const BG_MAGENTA_B:           &'static str = "'\033[105m'";
const BG_CYAN_B:              &'static str = "'\033[106m'";
const BG_WHITE_B:             &'static str = "'\033[107m'";

// Keys

const ESCAPE:                 &'static str = "'\033'";
const RETURN:                 &'static str = "'\r'";
const BACKSPACE:              &'static str = "'\x7f'";
const TAB:                    &'static str = "'\t'";
const TAB_SHIFT:              &'static str = "'\033[Z'";

// Arrows

const ARROW_UP:               &'static str = "'\033[A'";
const ARROW_DOWN:             &'static str = "'\033[B'";
const ARROW_LEFT:             &'static str = "'\033[D'";
const ARROW_RIGHT:            &'static str = "'\033[C'";
const ARROW_UP_SHIFT:         &'static str = "'\033[1;2A'";
const ARROW_DOWN_SHIFT:       &'static str = "'\033[1;2B'";
const ARROW_LEFT_SHIFT:       &'static str = "'\033[1;2D'";
const ARROW_RIGHT_SHIFT:      &'static str = "'\033[1;2C'";
const ARROW_UP_CTRL:          &'static str = "'\033[1;5A'";
const ARROW_DOWN_CTRL:        &'static str = "'\033[1;5B'";
const ARROW_LEFT_CTRL:        &'static str = "'\033[1;5D'";
const ARROW_RIGHT_CTRL:       &'static str = "'\033[1;5C'";
const ARROW_UP_CTRL_SHIFT:    &'static str = "'\033[1;6A'";
const ARROW_DOWN_CTRL_SHIFT:  &'static str = "'\033[1;6B'";
const ARROW_LEFT_CTRL_SHIFT:  &'static str = "'\033[1;6D'";
const ARROW_RIGHT_CTRL_SHIFT: &'static str = "'\033[1;6C'";
