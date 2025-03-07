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

    CUR_SAVE               string = "\x1b7"  // DEC
    CUR_RESTORE            string = "\x1b8"  // DEC 
    CUR_SAVE_SCO           string = "\x1b[s" // SCO
    CUR_RESTORE_SCO        string = "\x1b[u" // SCO

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
    ALT_MOD                string = "\x1b"
    // Adding escape in front of most key codes gives its alt variant.
    // ALT_MOD can be used instead of ESCAPE for clarity.

    TAB                    string = "\x09"
    RETURN                 string = "\x0d"
    BACKSPACE              string = "\x7f"
    SHIFT_TAB              string = "\x1b[Z"
    CTRL_BACKSPACE         string = "\x08"

    HOME                   string = "\x1b[H"
    END                    string = "\x1b[F"
    PG_UP                  string = "\x1b[5~"
    PG_DOWN                string = "\x1b[6~"
    DELETE                 string = "\x1b[3~"
    INSERT                 string = "\x1b[2~"

    CTRL_HOME              string = "\x1b[1;5H"
    CTRL_END               string = "\x1b[1;5F"
    CTRL_PG_UP             string = "\x1b[5;5~"
    CTRL_PG_DOWN           string = "\x1b[6;5~"
    CTRL_DELETE            string = "\x1b[3;5~"
    CTRL_INSERT            string = "\x1b[2;5~"

    // Arrows

    ARROW_UP               string = "\x1b[A"
    ARROW_DOWN             string = "\x1b[B"
    ARROW_LEFT             string = "\x1b[D"
    ARROW_RIGHT            string = "\x1b[C"
    SHIFT_ARROW_UP         string = "\x1b[1;2A"
    SHIFT_ARROW_DOWN       string = "\x1b[1;2B"
    SHIFT_ARROW_LEFT       string = "\x1b[1;2D"
    SHIFT_ARROW_RIGHT      string = "\x1b[1;2C"
    CTRL_ARROW_UP          string = "\x1b[1;5A"
    CTRL_ARROW_DOWN        string = "\x1b[1;5B"
    CTRL_ARROW_LEFT        string = "\x1b[1;5D"
    CTRL_ARROW_RIGHT       string = "\x1b[1;5C"
    CTRL_SHIFT_ARROW_UP    string = "\x1b[1;6A"
    CTRL_SHIFT_ARROW_DOWN  string = "\x1b[1;6B"
    CTRL_SHIFT_ARROW_LEFT  string = "\x1b[1;6D"
    CTRL_SHIFT_ARROW_RIGHT string = "\x1b[1;6C"

    // Function keys

    F1                     string = "\x1bOP"
    F2                     string = "\x1bOQ"
    F3                     string = "\x1bOR"
    F4                     string = "\x1bOS"
    F5                     string = "\x1b[15~"
    F6                     string = "\x1b[17~"
    F7                     string = "\x1b[18~"
    F8                     string = "\x1b[19~"
    F9                     string = "\x1b[20~"
    F10                    string = "\x1b[21~"
    F11                    string = "\x1b[23~"
    F12                    string = "\x1b[24~"

    // Control + letter

    CTRL_A                 string = "\x01"
    CTRL_B                 string = "\x02"
    CTRL_C                 string = "\x03"
    CTRL_D                 string = "\x04"
    CTRL_E                 string = "\x05"
    CTRL_F                 string = "\x06"
    CTRL_G                 string = "\x07"
    CTRL_H                 string = "\x08"
    CTRL_I                 string = "\x09"
    CTRL_J                 string = "\x0a"
    CTRL_K                 string = "\x0b"
    CTRL_L                 string = "\x0c"
    CTRL_M                 string = "\x0d"
    CTRL_N                 string = "\x0e"
    CTRL_O                 string = "\x0f"
    CTRL_P                 string = "\x10"
    CTRL_Q                 string = "\x11"
    CTRL_R                 string = "\x12"
    CTRL_S                 string = "\x13"
    CTRL_T                 string = "\x14"
    CTRL_U                 string = "\x15"
    CTRL_V                 string = "\x16"
    CTRL_W                 string = "\x17"
    CTRL_X                 string = "\x18"
    CTRL_Y                 string = "\x19"
    CTRL_Z                 string = "\x1a"
)