import os
import platform
import sys
import time

# working directory must be "/makespan-minimization"!

# run config on all instances (==.txt files) in benchmarks directory
# example config: "--bf --lpt --rr --rf --swap --ff --rf-configs , --swap-configs ,,"

if len(sys.argv) < 2:
    print(f"ERROR: Usage: {sys.argv[0]} \"makespan-minimization-args(without --path and --write)\"")
    exit(0)

args = sys.argv[1]

os.system('cargo build')
mm = ''
if 'Windows' in platform.platform():
    mm = 'target\debug\makespan-minimization.exe '
else:
    mm = 'target/debug/makespan-minimization '

start_time = time.time()

for file in os.listdir("./benchmarks"):
    if os.path.isfile('./benchmarks/' + file) and ".txt" in file:
        print("starting with input: '" + file + "'")

        os.system(mm + '--path ' + './benchmarks/' + file + ' ' + args + ' --write --write-separate-files')

        print("end with input: '" + file + "' -----------------------\n")

end_time = time.time()
print(f"time: {end_time - start_time} sec")
