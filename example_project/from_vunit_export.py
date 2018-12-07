"""
Create a vhdl_ls.toml file from a VUnit --export-json file
"""

import argparse
import json
import toml
from os.path import relpath, dirname, join
from glob import glob


def main():
    parser = argparse.ArgumentParser("Create a vhdl_ls.toml file from a VUnit --export-json file")
    parser.add_argument("json_file", nargs=1,
                        help="The input .json file")
    parser.add_argument("-o", "--output", default="vhdl_ls.toml",
                        help="The output vhdl_ls.toml file")

    args = parser.parse_args()

    with open(args.json_file[0], "r") as fptr:
        data = json.load(fptr)

    libraries = {}
    for source_file in data["files"]:
        file_name = source_file["file_name"]
        library_name = source_file["library_name"]

        if not library_name in libraries:
            libraries[library_name] = set()

        libraries[library_name].add(file_name)

    std_path = join(dirname(__file__), "vhdl_libraries", "2008", "std", "*.vhd")
    libraries["std"] = glob(std_path)
    assert libraries["std"], "Could not find STD library: %s" % std_path

    ieee_path = join(dirname(__file__), "ghdl", "libraries", "ieee2008", "*.vhdl")
    libraries["ieee"] = glob(ieee_path)
    assert libraries["ieee"], "Could not find IEEE library %s, have you run setup.sh?" % ieee_path

    with open(args.output, "w") as fptr:
        for key in libraries:
            libraries[key] = dict(files=[relpath(file_name, dirname(args.output))
                                         for file_name in libraries[key]])
        toml.dump(dict(libraries=libraries), fptr)



if __name__ == "__main__":
    main()
