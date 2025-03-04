# Control codes

BELL = '\007'

# Cursor control

CUR_HOME               = '\033[H'
CUR_SET                = '\033[{};{}H'
CUR_UP                 = '\033[{}A'
CUR_DOWN               = '\033[{}B'
CUR_RIGHT              = '\033[{}C'
CUR_LEFT               = '\033[{}D'
CUR_DOWN_BEG           = '\033[{}E'
CUR_UP_BEG             = '\033[{}F'
CUR_COL                = '\033[{}G'

CUR_UP_ONE             = '\033[1A'
CUR_DOWN_ONE           = '\033[1B'
CUR_RIGHT_ONE          = '\033[1C'
CUR_LEFT_ONE           = '\033[1D'

CUR_COL_HOME           = '\033[0G'

CUR_HIDE               = '\033[?25l'
CUR_SHOW               = '\033[?25h'

CUR_SAVE_DEC           = '\033 7' # DEC
CUR_RESTORE_DEC        = '\033 8' # DEC 
CUR_SAVE_SCO           = '\033[s' # SCO
CUR_RESTORE_SCO        = '\033[u' # SCO

# Screen operations

SCREEN_SAVE            = '\033[?47h'
RESTORE_SAVE           = '\033[?47l'

ERASE_SCREEN           = '\033[2J'
ERASE_LINE             = '\033[2K'
ERASE_TO_END           = '\033[0J'
ERASE_TO_LINE_END      = '\033[0J'

# Style

RESET                  = '\033[0m'
BOLD                   = '\033[1m'
DIM                    = '\033[2m'
ITALIC                 = '\033[3m'
UNDERLINE              = '\033[4m'
BLINK                  = '\033[5m'
STRIKE                 = '\033[9m'

# 256 colors

FG_ID                  = '\033[38;5;{}m'
BG_ID                  = '\033[48;5;{}m'

# True color

FG_RGB                 = '\033[38;2;{};{};{}m'
BG_RGB                 = '\033[48;2;{};{};{}m'

# 16 color codes

FG_BLACK               = '\033[30m'
FG_RED                 = '\033[31m'
FG_GREEN               = '\033[32m'
FG_YELLOW              = '\033[33m'
FG_BLUE                = '\033[34m'
FG_MAGENTA             = '\033[35m'
FG_CYAN                = '\033[36m'
FG_WHITE               = '\033[37m'
FG_DEFAULT             = '\033[39m'

BG_BLACK               = '\033[40m'
BG_RED                 = '\033[41m'
BG_GREEN               = '\033[42m'
BG_YELLOW              = '\033[43m'
BG_BLUE                = '\033[44m'
BG_MAGENTA             = '\033[45m'
BG_CYAN                = '\033[46m'
BG_WHITE               = '\033[47m'
BG_DEFAULT             = '\033[49m'

# Bright versions

FG_BLACK_B             = '\033[90m'
FG_RED_B               = '\033[91m'
FG_GREEN_B             = '\033[92m'
FG_YELLOW_B            = '\033[93m'
FG_BLUE_B              = '\033[94m'
FG_MAGENTA_B           = '\033[95m'
FG_CYAN_B              = '\033[96m'
FG_WHITE_B             = '\033[97m'

BG_BLACK_B             = '\033[100m'
BG_RED_B               = '\033[101m'
BG_GREEN_B             = '\033[102m'
BG_YELLOW_B            = '\033[103m'
BG_BLUE_B              = '\033[104m'
BG_MAGENTA_B           = '\033[105m'
BG_CYAN_B              = '\033[106m'
BG_WHITE_B             = '\033[107m'

# Keys

ESCAPE                 = '\033'
RETURN                 = '\r'
BACKSPACE              = '\x7f'
TAB                    = '\t'
TAB_SHIFT              = '\033[Z'

# Arrows

ARROW_UP               = '\033[A'
ARROW_DOWN             = '\033[B'
ARROW_LEFT             = '\033[D'
ARROW_RIGHT            = '\033[C'
ARROW_UP_SHIFT         = '\033[1;2A'
ARROW_DOWN_SHIFT       = '\033[1;2B'
ARROW_LEFT_SHIFT       = '\033[1;2D'
ARROW_RIGHT_SHIFT      = '\033[1;2C'
ARROW_UP_CTRL          = '\033[1;5A'
ARROW_DOWN_CTRL        = '\033[1;5B'
ARROW_LEFT_CTRL        = '\033[1;5D'
ARROW_RIGHT_CTRL       = '\033[1;5C'
ARROW_UP_CTRL_SHIFT    = '\033[1;6A'
ARROW_DOWN_CTRL_SHIFT  = '\033[1;6B'
ARROW_LEFT_CTRL_SHIFT  = '\033[1;6D'
ARROW_RIGHT_CTRL_SHIFT = '\033[1;6C'
