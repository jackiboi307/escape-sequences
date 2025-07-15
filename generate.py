import os
import shutil
import tomllib
import traceback

with open("config.toml", "rb") as file:
    config = tomllib.load(file)

def format(string, **kwargs):
    for kwarg in kwargs:
        string = string.replace("{" + kwarg + "}", kwargs[kwarg])
    return string

langs = []

for filename in os.listdir("langs"):
    with open(os.path.join("langs", filename), "rb") as file:
        langs.append(tomllib.load(file))

datas = {}

for filename in os.listdir("data"):
    name = filename.split(".")[0]
    datas[name] = []
    data = datas[name]

    with open(os.path.join("data", filename)) as file:
        lines = file.read().splitlines()

        for line in lines:
            if line == "":
                data.append({
                    "type": "blank"
                })

            elif line[0] == "#":
                data.append({
                    "type": "comment",
                    "text": line[1:]
                })

            else:
                spl = line.split()
                key = spl[0]
                values = spl[1:]
                arr = len(values) > 1

                if arr:
                    data.append({
                        "type": "arr",
                        "key": key,
                        "values": values
                    })

                else:
                    data.append({
                        "type": "code",
                        "key": key,
                        "value": values[0]
                    })

os.makedirs(config["output_dir"], exist_ok=True)

for lang in langs:
    print(f"Generating {lang['name']} file...")

    try:
        files = {}

        for name in datas:
            data = datas[name]

            files[name] = [[lang["file_beg"]]] if lang["file_beg"] != "" else []
            output = files[name]

            for row in data:
                t = row["type"]

                if t == "blank":
                    output.append([])
                
                elif t == "comment":
                    output.append([lang["comment"] + row["text"]])

                else:
                    args = dict(key=row["key"])

                    if t == "code":
                        value = row["value"]
                        value.replace("{}", lang["format"])
                        args = args | dict(value=value)
                        output.append([
                            format(lang["code_beg"], **args),
                            format(lang["code_end"], **args)
                        ])

                    elif t == "arr":
                        values = row["values"]
                        values = [value.replace("{}", lang["format"]) for value in values]
                        output.append([
                            format(lang["arr_beg"], **args),
                            format(lang["arr_mid"], **args) + \
                                lang["arr_sep"].join(values) + \
                            format(lang["arr_end"], **args)
                        ])

        longest_beg = max([len(line[0]) for line in sum(files.values(), []) if len(line) > 1])

        for name in files:
            output = files[name]

            if config["align"]:
                for line in output:
                    if len(line) > 1:
                        line.insert(1, " " * (longest_beg - len(line[0])))

            output.append([lang["file_end"]])
            output = "\n".join(["".join(line) for line in output])

            os.makedirs(os.path.join(config["output_dir"], lang["name"]), exist_ok=True)

            with open(os.path.join(
                    config["output_dir"],
                    lang["name"],
                    name + "." + lang["ext"]), "w") as file:
                file.write(output)
    
    except Exception as e:
        traceback.print_exc()
        # try:
        #     shutil.rmtree(os.path.join(config["output_dir"], lang["name"]))
        # except FileNotFoundError:
        #     pass

