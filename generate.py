COMMENT = 0
CONST = 1
CONST_INLINE_COMMENT = 2

RUST_FULL = (
    "rs",

    "#![allow(dead_code)] // disable warnings for unused code\n\n"
    "type StringType = &'static str;\n\n",
    "",

    (
        "// {}",
        "pub const {}:{} StringType = \"{}\";",
        "// {}"
    ),

    "{}"
)

RUST_COPY = (
    "rs",

    "",
    "",

    (
        "// {}",
        "const {}:{} &'static str = \"{}\";",
        "// {}"
    ),

    "{}"
)

# Uses the gofmt-preferred const ( ... ) syntax
GO_FULL = (
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

# Uses repetitive const syntax
GO_COPY = (
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

PY_COPY = (
    "py",

    "",
    "",

    (
        "# {}",
        "{}{} = \"{}\"",
        "# {}"
    ),

    "{}"
)

# Formatting rules
LONGEST = 22 # NOTE: This constant must be updated if adding longer keys to const.py
INLINE_COMMENT_SPACING = " "

FORMATS = (
    (RUST_FULL, RUST_COPY),
    (GO_FULL, GO_COPY),
    (None, PY_COPY),
)

def py_interpret(row, use_copy):
    if row[0] == "#":
        return COMMENT, row[2:]

    else:
        key, code = row.split("=", 1)

        key = key.rstrip()
        code = code.lstrip()

        inline_comment = code[code.index("'", 1) + 1:]

        if inline_comment == "" or use_copy:
            if use_copy:
                if "#" in code:
                    code = code[:code.index("#")-1]

            return CONST, key, code[1:-1]

        else:
            return CONST_INLINE_COMMENT, key, \
                   code[1:code.index("'", 1) - 1], inline_comment[3:]

with open("for_importing/escape_sequences.py") as origin_file:
    lines = origin_file.read().splitlines()

for use_copy in (False, True):
    longest = 0 if use_copy else LONGEST

    for i in FORMATS:
        i = i[1] if use_copy else i[0]
        if i is None:
            continue

        output = i[1]

        for row in lines:
            if row.isspace() or row == "":
                output += row + "\n"

            else:
                interpreted = py_interpret(row, use_copy)

                if interpreted[0] == COMMENT:
                    new = i[3][COMMENT].format(interpreted[1])

                elif interpreted[0] == CONST:
                    new = i[3][CONST].format(interpreted[1],
                                             " " * (longest - len(interpreted[1])),
                                             interpreted[2].replace("{}", i[4]))

                elif interpreted[0] == CONST_INLINE_COMMENT:
                    new = i[3][CONST].format(interpreted[1],
                                             " " * (longest - len(interpreted[1])),
                                             interpreted[2].replace("{}", i[4])) + \
                          INLINE_COMMENT_SPACING + i[3][CONST_INLINE_COMMENT].format(interpreted[3])

                output += new + "\n"

        output += i[2]

        directory = "for_copying" if use_copy else "for_importing"
        with open(f"{directory}/escape_sequences.{i[0]}", "w") as output_file:
            output_file.write(output)
