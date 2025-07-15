# Control codes

BELL = '\x07'

# Cursor control

CUR_HOME = '\x1b[H'
CUR_SET = '\x1b[{};{}H'
CUR_UP = '\x1b[{}A'
CUR_DOWN = '\x1b[{}B'
CUR_RIGHT = '\x1b[{}C'
CUR_LEFT = '\x1b[{}D'
CUR_DOWN_BEG = '\x1b[{}E'
CUR_UP_BEG = '\x1b[{}F'
CUR_COL = '\x1b[{}G'

CUR_UP_ONE = '\x1b[1A'
CUR_DOWN_ONE = '\x1b[1B'
CUR_RIGHT_ONE = '\x1b[1C'
CUR_LEFT_ONE = '\x1b[1D'

CUR_COL_HOME = '\x1b[0G'

CUR_HIDE = '\x1b[?25l'
CUR_SHOW = '\x1b[?25h'

CUR_SAVE = '\x1b7'
CUR_RESTORE = '\x1b8'
CUR_SAVE_SCO = '\x1b[s'
CUR_RESTORE_SCO = '\x1b[u'

# Screen operations

SCREEN_SAVE = '\x1b[?47h'
RESTORE_SAVE = '\x1b[?47l'

ERASE_SCREEN = '\x1b[2J'
ERASE_LINE = '\x1b[2K'
ERASE_TO_END = '\x1b[0J'
ERASE_TO_LINE_END = '\x1b[0J'

# Style

RESET = '\x1b[0m'
BOLD = '\x1b[1m'
DIM = '\x1b[2m'
ITALIC = '\x1b[3m'
UNDERLINE = '\x1b[4m'
BLINK = '\x1b[5m'
STRIKE = '\x1b[9m'

# 256 colors

FG_ID = '\x1b[38;5;{}m'
BG_ID = '\x1b[48;5;{}m'

# True color

FG_RGB = '\x1b[38;2;{};{};{}m'
BG_RGB = '\x1b[48;2;{};{};{}m'

# 16 color codes

FG_BLACK = '\x1b[30m'
FG_RED = '\x1b[31m'
FG_GREEN = '\x1b[32m'
FG_YELLOW = '\x1b[33m'
FG_BLUE = '\x1b[34m'
FG_MAGENTA = '\x1b[35m'
FG_CYAN = '\x1b[36m'
FG_WHITE = '\x1b[37m'
FG_DEFAULT = '\x1b[39m'

BG_BLACK = '\x1b[40m'
BG_RED = '\x1b[41m'
BG_GREEN = '\x1b[42m'
BG_YELLOW = '\x1b[43m'
BG_BLUE = '\x1b[44m'
BG_MAGENTA = '\x1b[45m'
BG_CYAN = '\x1b[46m'
BG_WHITE = '\x1b[47m'
BG_DEFAULT = '\x1b[49m'

# Bright versions

FG_BLACK_B = '\x1b[90m'
FG_RED_B = '\x1b[91m'
FG_GREEN_B = '\x1b[92m'
FG_YELLOW_B = '\x1b[93m'
FG_BLUE_B = '\x1b[94m'
FG_MAGENTA_B = '\x1b[95m'
FG_CYAN_B = '\x1b[96m'
FG_WHITE_B = '\x1b[97m'

BG_BLACK_B = '\x1b[100m'
BG_RED_B = '\x1b[101m'
BG_GREEN_B = '\x1b[102m'
BG_YELLOW_B = '\x1b[103m'
BG_BLUE_B = '\x1b[104m'
BG_MAGENTA_B = '\x1b[105m'
BG_CYAN_B = '\x1b[106m'
BG_WHITE_B = '\x1b[107m'
