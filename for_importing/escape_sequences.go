const (

    // Control codes

    BELL                   string = "\x07"

    // Cursor control

    CUR_HOME               string = "\x1b[H"
    CUR_SET                string = "\x1b[%s;%sH"
    CUR_UP                 string = "\x1b[%sA"
    CUR_DOWN               string = "\x1b[%sB"
    CUR_RIGHT              string = "\x1b[%sC"
    CUR_LEFT               string = "\x1b[%sD"
    CUR_DOWN_BEG           string = "\x1b[%sE"
    CUR_UP_BEG             string = "\x1b[%sF"
    CUR_COL                string = "\x1b[%sG"

    CUR_UP_ONE             string = "\x1b[1A"
    CUR_DOWN_ONE           string = "\x1b[1B"
    CUR_RIGHT_ONE          string = "\x1b[1C"
    CUR_LEFT_ONE           string = "\x1b[1D"

    CUR_COL_HOME           string = "\x1b[0G"

    CUR_HIDE               string = "\x1b[?25l"
    CUR_SHOW               string = "\x1b[?25h"

    CUR_SAVE_DEC           string = "\x1b " // DEC
    CUR_RESTORE_DEC        string = "\x1b " // DEC 
    CUR_SAVE_SCO           string = "\x1b[" // SCO
    CUR_RESTORE_SCO        string = "\x1b[" // SCO

    // Screen operations

    SCREEN_SAVE            string = "\x1b[?47h"
    RESTORE_SAVE           string = "\x1b[?47l"

    ERASE_SCREEN           string = "\x1b[2J"
    ERASE_LINE             string = "\x1b[2K"
    ERASE_TO_END           string = "\x1b[0J"
    ERASE_TO_LINE_END      string = "\x1b[0J"

    // Style

    RESET                  string = "\x1b[0m"
    BOLD                   string = "\x1b[1m"
    DIM                    string = "\x1b[2m"
    ITALIC                 string = "\x1b[3m"
    UNDERLINE              string = "\x1b[4m"
    BLINK                  string = "\x1b[5m"
    STRIKE                 string = "\x1b[9m"

    // 256 colors

    FG_ID                  string = "\x1b[38;5;%sm"
    BG_ID                  string = "\x1b[48;5;%sm"

    // True color

    FG_RGB                 string = "\x1b[38;2;%s;%s;%sm"
    BG_RGB                 string = "\x1b[48;2;%s;%s;%sm"

    // 16 color codes

    FG_BLACK               string = "\x1b[30m"
    FG_RED                 string = "\x1b[31m"
    FG_GREEN               string = "\x1b[32m"
    FG_YELLOW              string = "\x1b[33m"
    FG_BLUE                string = "\x1b[34m"
    FG_MAGENTA             string = "\x1b[35m"
    FG_CYAN                string = "\x1b[36m"
    FG_WHITE               string = "\x1b[37m"
    FG_DEFAULT             string = "\x1b[39m"

    BG_BLACK               string = "\x1b[40m"
    BG_RED                 string = "\x1b[41m"
    BG_GREEN               string = "\x1b[42m"
    BG_YELLOW              string = "\x1b[43m"
    BG_BLUE                string = "\x1b[44m"
    BG_MAGENTA             string = "\x1b[45m"
    BG_CYAN                string = "\x1b[46m"
    BG_WHITE               string = "\x1b[47m"
    BG_DEFAULT             string = "\x1b[49m"

    // Bright versions

    FG_BLACK_B             string = "\x1b[90m"
    FG_RED_B               string = "\x1b[91m"
    FG_GREEN_B             string = "\x1b[92m"
    FG_YELLOW_B            string = "\x1b[93m"
    FG_BLUE_B              string = "\x1b[94m"
    FG_MAGENTA_B           string = "\x1b[95m"
    FG_CYAN_B              string = "\x1b[96m"
    FG_WHITE_B             string = "\x1b[97m"

    BG_BLACK_B             string = "\x1b[100m"
    BG_RED_B               string = "\x1b[101m"
    BG_GREEN_B             string = "\x1b[102m"
    BG_YELLOW_B            string = "\x1b[103m"
    BG_BLUE_B              string = "\x1b[104m"
    BG_MAGENTA_B           string = "\x1b[105m"
    BG_CYAN_B              string = "\x1b[106m"
    BG_WHITE_B             string = "\x1b[107m"

    // Keys

    ESCAPE                 string = "\x1b"
    RETURN                 string = "\x0d"
    BACKSPACE              string = "\x7f"
    TAB                    string = "\x09"
    TAB_SHIFT              string = "\x1b[Z"

    // Arrows

    ARROW_UP               string = "\x1b[A"
    ARROW_DOWN             string = "\x1b[B"
    ARROW_LEFT             string = "\x1b[D"
    ARROW_RIGHT            string = "\x1b[C"
    ARROW_UP_SHIFT         string = "\x1b[1;2A"
    ARROW_DOWN_SHIFT       string = "\x1b[1;2B"
    ARROW_LEFT_SHIFT       string = "\x1b[1;2D"
    ARROW_RIGHT_SHIFT      string = "\x1b[1;2C"
    ARROW_UP_CTRL          string = "\x1b[1;5A"
    ARROW_DOWN_CTRL        string = "\x1b[1;5B"
    ARROW_LEFT_CTRL        string = "\x1b[1;5D"
    ARROW_RIGHT_CTRL       string = "\x1b[1;5C"
    ARROW_UP_CTRL_SHIFT    string = "\x1b[1;6A"
    ARROW_DOWN_CTRL_SHIFT  string = "\x1b[1;6B"
    ARROW_LEFT_CTRL_SHIFT  string = "\x1b[1;6D"
    ARROW_RIGHT_CTRL_SHIFT string = "\x1b[1;6C"
)