const (

    // Control codes

    BELL                   string = "'\007'"

    // Cursor control

    CUR_HOME               string = "'\033[H'"
    CUR_SET                string = "'\033[%s;%sH'"
    CUR_UP                 string = "'\033[%sA'"
    CUR_DOWN               string = "'\033[%sB'"
    CUR_RIGHT              string = "'\033[%sC'"
    CUR_LEFT               string = "'\033[%sD'"
    CUR_DOWN_BEG           string = "'\033[%sE'"
    CUR_UP_BEG             string = "'\033[%sF'"
    CUR_COL                string = "'\033[%sG'"

    CUR_UP_ONE             string = "'\033[1A'"
    CUR_DOWN_ONE           string = "'\033[1B'"
    CUR_RIGHT_ONE          string = "'\033[1C'"
    CUR_LEFT_ONE           string = "'\033[1D'"

    CUR_COL_HOME           string = "'\033[0G'"

    CUR_HIDE               string = "'\033[?25l'"
    CUR_SHOW               string = "'\033[?25h'"

    CUR_SAVE_DEC           string = "'\033 7" // DEC
    CUR_RESTORE_DEC        string = "'\033 8" // DEC 
    CUR_SAVE_SCO           string = "'\033[s" // SCO
    CUR_RESTORE_SCO        string = "'\033[u" // SCO

    // Screen operations

    SCREEN_SAVE            string = "'\033[?47h'"
    RESTORE_SAVE           string = "'\033[?47l'"

    ERASE_SCREEN           string = "'\033[2J'"
    ERASE_LINE             string = "'\033[2K'"
    ERASE_TO_END           string = "'\033[0J'"
    ERASE_TO_LINE_END      string = "'\033[0J'"

    // Style

    RESET                  string = "'\033[0m'"
    BOLD                   string = "'\033[1m'"
    DIM                    string = "'\033[2m'"
    ITALIC                 string = "'\033[3m'"
    UNDERLINE              string = "'\033[4m'"
    BLINK                  string = "'\033[5m'"
    STRIKE                 string = "'\033[9m'"

    // 256 colors

    FG_ID                  string = "'\033[38;5;%sm'"
    BG_ID                  string = "'\033[48;5;%sm'"

    // True color

    FG_RGB                 string = "'\033[38;2;%s;%s;%sm'"
    BG_RGB                 string = "'\033[48;2;%s;%s;%sm'"

    // 16 color codes

    FG_BLACK               string = "'\033[30m'"
    FG_RED                 string = "'\033[31m'"
    FG_GREEN               string = "'\033[32m'"
    FG_YELLOW              string = "'\033[33m'"
    FG_BLUE                string = "'\033[34m'"
    FG_MAGENTA             string = "'\033[35m'"
    FG_CYAN                string = "'\033[36m'"
    FG_WHITE               string = "'\033[37m'"
    FG_DEFAULT             string = "'\033[39m'"

    BG_BLACK               string = "'\033[40m'"
    BG_RED                 string = "'\033[41m'"
    BG_GREEN               string = "'\033[42m'"
    BG_YELLOW              string = "'\033[43m'"
    BG_BLUE                string = "'\033[44m'"
    BG_MAGENTA             string = "'\033[45m'"
    BG_CYAN                string = "'\033[46m'"
    BG_WHITE               string = "'\033[47m'"
    BG_DEFAULT             string = "'\033[49m'"

    // Bright versions

    FG_BLACK_B             string = "'\033[90m'"
    FG_RED_B               string = "'\033[91m'"
    FG_GREEN_B             string = "'\033[92m'"
    FG_YELLOW_B            string = "'\033[93m'"
    FG_BLUE_B              string = "'\033[94m'"
    FG_MAGENTA_B           string = "'\033[95m'"
    FG_CYAN_B              string = "'\033[96m'"
    FG_WHITE_B             string = "'\033[97m'"

    BG_BLACK_B             string = "'\033[100m'"
    BG_RED_B               string = "'\033[101m'"
    BG_GREEN_B             string = "'\033[102m'"
    BG_YELLOW_B            string = "'\033[103m'"
    BG_BLUE_B              string = "'\033[104m'"
    BG_MAGENTA_B           string = "'\033[105m'"
    BG_CYAN_B              string = "'\033[106m'"
    BG_WHITE_B             string = "'\033[107m'"

    // Keys

    ESCAPE                 string = "'\033'"
    RETURN                 string = "'\r'"
    BACKSPACE              string = "'\x7f'"
    TAB                    string = "'\t'"
    TAB_SHIFT              string = "'\033[Z'"

    // Arrows

    ARROW_UP               string = "'\033[A'"
    ARROW_DOWN             string = "'\033[B'"
    ARROW_LEFT             string = "'\033[D'"
    ARROW_RIGHT            string = "'\033[C'"
    ARROW_UP_SHIFT         string = "'\033[1;2A'"
    ARROW_DOWN_SHIFT       string = "'\033[1;2B'"
    ARROW_LEFT_SHIFT       string = "'\033[1;2D'"
    ARROW_RIGHT_SHIFT      string = "'\033[1;2C'"
    ARROW_UP_CTRL          string = "'\033[1;5A'"
    ARROW_DOWN_CTRL        string = "'\033[1;5B'"
    ARROW_LEFT_CTRL        string = "'\033[1;5D'"
    ARROW_RIGHT_CTRL       string = "'\033[1;5C'"
    ARROW_UP_CTRL_SHIFT    string = "'\033[1;6A'"
    ARROW_DOWN_CTRL_SHIFT  string = "'\033[1;6B'"
    ARROW_LEFT_CTRL_SHIFT  string = "'\033[1;6D'"
    ARROW_RIGHT_CTRL_SHIFT string = "'\033[1;6C'"
)