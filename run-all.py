import os
import sys

# run config on all instances (==.txt files) in benchmarks directory
# example config: "--bf --lpt --rr --rf --swap --ff --rf-configs , --swap-configs ,,"

if len(sys.argv) < 2:
    print(f"ERROR: Usage: {sys.argv[0]} \"makespan-minimization-args(without --path and --write)\"")
    exit(0)

args = sys.argv[1]

os.system('cargo build')

for file in os.listdir("./benchmarks"):
    if os.path.isfile('./benchmarks/' + file) and ".txt" in file:
        print("starting with input: '" + file + "'")
        os.system(
            'target\debug\makespan-minimization.exe --path ' + './benchmarks/' + file + ' ' + args + ' --write --write-separate-files"')
        # os.system('target\debug\makespan-minimization.exe --path ' + './benchmarks/' + file + ' --bf --lpt --rr --rf --swap --ff --rf-configs , --swap-configs ,, --write --write-separate-files')
        # os.system('cargo run --bin makespan-minimization -- --path ' + file + ' --bf --lpt --rr --rf --swap --ff --rf-configs , --swap-configs ,, --write --write-separate-files')
        print("end with input: '" + file + "' -----------------------\n")
