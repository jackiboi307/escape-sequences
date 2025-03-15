from variables import *

def gen_rust(data, kind):
    res = ""

    if kind == IMPORTING:
        res += \
"""#![allow(dead_code)] // disable warnings for unused code

type StringType = &'static str;

"""

    for i in data:
        match i["type"]:
            case "blank":
                pass

            case "comment":
                res += "// {}".format(i["text"])

            case "str" | "arr":
                res += '{}const {}:{}'.format(
                        "pub " if kind == IMPORTING else "",

                        i["key"],
                        i["space"] if kind == IMPORTING else " ",
                )

                match i["type"]:
                    case "str":
                        res += "StringType" if kind == IMPORTING else "&'static str"
                        res += ' = "{}";'.format(i["string"])

                    case "arr":
                        res += ("[StringType; {}]" if kind == IMPORTING else "[&'static str; {}]").format(len(i["arr"]))
                        res += " = [{}];".format(", ".join(f'"{s}"' for s in i["arr"]))

                if i["comment"] is not None and kind == IMPORTING:
                    res += "{}// {}".format(i["comment_spacing"], i["comment"])

        res += "\n"

    res = res.rstrip() # remove trailing whitespace
    if kind == COPYING:
        res += \
"""

use
// This invalid row exists solely to make GitHub correctly classify this file as a Rust file, and not RenderScript.
// You're not supposed to compile this file anyway - see for_importing/escape_sequences.rs if that's what you are looking for.
// The pattern for Rust files can be found here: https://github.com/github-linguist/linguist/blob/80f3531e8a1014a23f4606458e5a528053ed3cac/lib/linguist/heuristics.yml#L503-L510"""
    res += "\n" # add newline

    return res

def gen_go(data, kind):
    res = ""

    if kind == IMPORTING:
        res += \
"""// Maybe this file would be prettier with const ( ... ) and var ( ... ) syntax,
// but it would be hard to implement without fucking up the arrays.

type StringType = string

"""

    for i in data:
        match i["type"]:
            case "blank":
                pass

            case "comment":
                res += "// {}".format(i["text"])

            case "str" | "arr":
                res += "{} {}{}{}= {};{}".format(
                    "const" if i["type"] == "str" else "var  " if kind == IMPORTING else "var",

                    i["key"],
                    i["space"] if kind == IMPORTING else " ",

                    ("string " if kind == COPYING else "StringType ") if i["type"] == "str" else "",

                    '"{}"'.format(i["string"]).replace("{}", "%s") if i["type"] == "str" else \
                        "[]" + ("string" if kind == COPYING else "StringType") + "{" + ", ".join(f'"{s}"'.replace("{}", "%s") for s in i["arr"]) + "}",

                    "" if i["comment"] is None or kind == COPYING else \
                        "{}// {}".format(i["comment_spacing"], i["comment"])
                )

        res += "\n"

    return res

def gen_python(data, kind):
    res = ""

    for i in data:
        match i["type"]:
            case "blank":
                pass

            case "comment":
                res += "# {}".format(i["text"])

            case "str" | "arr":
                res += "{}{}= {}{}".format(
                    i["key"],
                    i["space"] if kind == IMPORTING else " ",

                    "'{}'".format(i["string"]) if i["type"] == "str" else \
                        "[{}]".format(", ".join(f"'{s}'" for s in i["arr"])),

                    i["comment_spacing"] + "# " + i["comment"] if i["comment"] is not None and kind == IMPORTING else ""
                )

        res += "\n"

    return res
