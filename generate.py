COMMENT = 0
CONST = 1
CONST_INLINE_COMMENT = 2

RUST = (
    "rs",

    "#![allow(...)]\n\n",
    "",

    (
        "// {}",
        "const {}:{} &'static str = \"{}\";",
        "// {}"
    ),

    "{}"
)

# Uses repetitive const syntax
GO_1 = (
    "go",

    "",
    "",

    (
        "// {}",
        "const {}{} string = \"{}\"",
        "// {}"
    ),

    "%s"
)

# Uses the gofmt-preferred const ( ... ) syntax
GO_2 = (
    "go",

    "const (\n\n",
    ")",

    (
        "    // {}",
        "    {}{} string = \"{}\"",
        "// {}"
    ),

    "%s"
)

# Formatting rules
USE_EVEN_FORMATTING = True
LONGEST = 22 if USE_EVEN_FORMATTING else 0 # NOTE: This constant must be updated if adding longer keys to const.py
INLINE_COMMENT_SPACING = " "

GO = GO_2 if USE_EVEN_FORMATTING else GO_1

FORMATS = (
    RUST,
    GO,
)

def py_interpret(row):
    if row[0] == "#":
        return COMMENT, row[2:]

    else:
        key, code = row.split("=", 1)

        key = key.rstrip()
        code = code.lstrip()

        inline_comment = code[code.index("'", 1) + 1:]

        if inline_comment == "" or not USE_EVEN_FORMATTING:
            return CONST, key, code

        else:
            return CONST_INLINE_COMMENT, key, code[:code.index("'", 1)], inline_comment[3:]

with open("const_files/const.py") as origin_file:
    lines = origin_file.read().splitlines()

for i in FORMATS:
    output = i[1]

    for row in lines:
        if row.isspace() or row == "":
            output += row + "\n"

        else:
            interpreted = py_interpret(row)

            if interpreted[0] == COMMENT:
                new = i[3][COMMENT].format(interpreted[1])

            elif interpreted[0] == CONST:
                new = i[3][CONST].format(interpreted[1],
                                         " " * (LONGEST - len(interpreted[1])),
                                         interpreted[2].replace("{}", i[4]))

            elif interpreted[0] == CONST_INLINE_COMMENT:
                new = i[3][CONST].format(interpreted[1],
                                         " " * (LONGEST - len(interpreted[1])),
                                         interpreted[2].replace("{}", i[4])) + \
                      INLINE_COMMENT_SPACING + i[3][CONST_INLINE_COMMENT].format(interpreted[3])

            output += new + "\n"

    output += i[2]

    with open(f"const_files/const.{i[0]}", "w") as output_file:
        output_file.write(output)
