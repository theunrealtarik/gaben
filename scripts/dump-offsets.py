
import subprocess
import os
import re

current_dir = os.getcwd()
dumper_path = os.path.join(current_dir, "external", "cs2-dumper.exe")

# subprocess.run([dumper_path, "-o", "./src/generated/"])

directory_path = os.path.join(current_dir, "src", "generated", "offsets")
module_path = os.path.join(
    current_dir, "src", "generated", "offsets", "mod.rs")
module_content = ""

if not os.path.exists(directory_path):
    os.makedirs(directory_path)

if not os.path.exists(module_path):
    with open(module_path, "w"):
        pass

for filename in os.listdir(directory_path):
    filepath = os.path.join(directory_path, filename)

    if not filename.endswith(".rs"):
        os.remove(filepath)
    else:
        if filename != "mod.rs":
            print(filename)
            module_content += "pub mod " + filename.split(".")[0] + ";\n"

    if re.match(r"^(.+)\.dll\.rs$", filename):
        new_filename = re.sub(r"\.dll\.rs$", r".rs", filename)
        new_filepath = os.path.join(directory_path, new_filename)

        os.rename(filepath, new_filepath)

with open(module_path, "w") as module:
    module.write(module_content)
