from variables import *
from langs import *

import re
import os

if DEBUG:
    import pprint

targets = (
    (IMPORTING, "rs", gen_rust),
    (COPYING,   "rs", gen_rust),
    (IMPORTING, "go", gen_go),
    (COPYING,   "go", gen_go),
    (COPYING,   "py", gen_python),
)

row_str = re.compile(r"^([a-zA-Z0-9_]+)(\s*)=\s*'([^']*)'(?:(\s*)#\s*(.*))?")
row_arr = re.compile(r"^([a-zA-Z0-9_]+)(\s*)=\s*(\[(?:\s*'[^']*'\s*(?:,\s*)?)*\])(?:(\s*)#\s*(.*))?")
comment = re.compile(r"^\s*#\s*(.*)")

data = []

with open("for_importing/escape_sequences.py") as file:
    lines = file.read().splitlines()

error = False

for line in lines:
    if line == "" or line.isspace():
        data.append({
            "type": "blank"
        })

    elif match := row_str.match(line):
        data.append({
            "type":    "str",
            "key":     match.group(1),
            "space":   match.group(2),
            "string":  match.group(3),
            "comment": match.group(5).rstrip() if match.group(5) is not None else None,
            "comment_spacing": match.group(4) if match.group(4) is not None else None
        })

    elif match := row_arr.match(line):
        data.append({
            "type":   "arr",
            "key":     match.group(1),
            "space":   match.group(2),
            "arr":     [repr(i)[1:-1] for i in eval(match.group(3))], # very ugly!
            "comment": match.group(5).rstrip() if match.group(5) is not None else None,
            "comment_spacing": match.group(4) if match.group(4) is not None else None
        })

    elif match := comment.match(line):
        data.append({
            "type": "comment",
            "text":  match.group(1)
        })

    else:
        error = True
        print(f"Error parsing row:\n{line}\n")

if DEBUG:
    pprint.pp(data)

if error:
    print("Errors encountered! Not continuing.")
    exit(1)

for target in targets:
    with open(os.path.join(PATHS[target[0]], FILENAME + "." + target[1]), "w") as file:
        file.write(target[2](data, target[0]))
