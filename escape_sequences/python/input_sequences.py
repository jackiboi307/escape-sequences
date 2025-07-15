# Keys

ESCAPE = '\x1b'
ALT = '\x1b'
# Adding escape in front of most key codes gives its alt variant.
# ALT can be used instead of ESCAPE for clarity.

TAB = '\x09'
RETURN = '\x0d'
BACKSPACE = '\x7f'
SHIFT_TAB = '\x1b[Z'
CTRL_BACKSPACE = '\x08'

HOME = ['\x1b[1', '\x1b[H']
END = ['\x1b[4', '\x1b[F']
PG_UP = '\x1b[5~'
PG_DOWN = '\x1b[6~'
DELETE = '\x1b[3~'
INSERT = '\x1b[2~'

# Arrows

ARROW_UP = '\x1b[A'
ARROW_DOWN = '\x1b[B'
ARROW_LEFT = '\x1b[D'
ARROW_RIGHT = '\x1b[C'
SHIFT_ARROW_UP = '\x1b[1;2A'
SHIFT_ARROW_DOWN = '\x1b[1;2B'
SHIFT_ARROW_LEFT = '\x1b[1;2D'
SHIFT_ARROW_RIGHT = '\x1b[1;2C'
CTRL_ARROW_UP = '\x1b[1;5A'
CTRL_ARROW_DOWN = '\x1b[1;5B'
CTRL_ARROW_LEFT = '\x1b[1;5D'
CTRL_ARROW_RIGHT = '\x1b[1;5C'
CTRL_SHIFT_ARROW_UP = '\x1b[1;6A'
CTRL_SHIFT_ARROW_DOWN = '\x1b[1;6B'
CTRL_SHIFT_ARROW_LEFT = '\x1b[1;6D'
CTRL_SHIFT_ARROW_RIGHT = '\x1b[1;6C'

# Function keys

F1 = ['\x1bOP', '\x1b[11~']
F2 = ['\x1bOQ', '\x1b[12~']
F3 = ['\x1bOR', '\x1b[13~']
F4 = ['\x1bOS', '\x1b[14~']
F5 = '\x1b[15~'
F6 = '\x1b[17~'
F7 = '\x1b[18~'
F8 = '\x1b[19~'
F9 = '\x1b[20~'
F10 = '\x1b[21~'
F11 = '\x1b[23~'
F12 = '\x1b[24~'

# Control + letter

CTRL_A = '\x01'
CTRL_B = '\x02'
CTRL_C = '\x03'
CTRL_D = '\x04'
CTRL_E = '\x05'
CTRL_F = '\x06'
CTRL_G = '\x07'
CTRL_H = '\x08'
CTRL_I = '\x09'
CTRL_J = '\x0a'
CTRL_K = '\x0b'
CTRL_L = '\x0c'
CTRL_M = '\x0d'
CTRL_N = '\x0e'
CTRL_O = '\x0f'
CTRL_P = '\x10'
CTRL_Q = '\x11'
CTRL_R = '\x12'
CTRL_S = '\x13'
CTRL_T = '\x14'
CTRL_U = '\x15'
CTRL_V = '\x16'
CTRL_W = '\x17'
CTRL_X = '\x18'
CTRL_Y = '\x19'
CTRL_Z = '\x1a'
