import os
import subprocess
import sys

##used for validate-all.py!

# is quick&dirty aber funktioniert haha
if len(sys.argv) < 3:
    print(f"Usage: {sys.argv[0]} input_file output_file_dir")
    exit(0)

input_file = sys.argv[1]
directory_name = sys.argv[2]

for file_name in os.listdir(directory_name):
    i = os.path.join(directory_name, file_name)
    if os.path.isfile(i) and "solution.txt" in i:
        print(i)
        print(subprocess.Popen(
            ("py data/validate-solution.py " + input_file + " " + i),
            shell=True, stdout=subprocess.PIPE).stdout.read())
        print("\n")
