import os
import subprocess
import sys

# is quick&dirty aber funktioniert haha
if len(sys.argv) < 2:
    print(f"Usage: {sys.argv[0]} instancefile")
    exit(0)

instancefile = sys.argv[1]

directory_name = "."

for file_name in os.listdir(directory_name):
    i = os.path.join(directory_name, file_name)
    if os.path.isfile(i) and "solution.txt" in i:
        print(i)
        print(subprocess.Popen(
            ("py ./validate-solution.py " + instancefile + " " + i),
            shell=True, stdout=subprocess.PIPE).stdout.read())
        print("\n")
