import os
import platform
import sys
import time
from threading import Thread


# working directory must be "/makespan-minimization"!

# run config on all instances (==.txt files) in benchmarks directory
# example config: "--bf --lpt --rr --rf --swap --ff --rf-configs , --swap-configs ,,"

def run_all():
    if len(sys.argv) < 4:
        print(f"ERROR: Usage: {sys.argv[0]} \"makespan-minimization-args(without --path and --write)\" #runs name")
        exit(0)

    args = sys.argv[1]
    runs = int(sys.argv[2])
    name = sys.argv[3]

    os.system('cargo build')
    mm = ''
    if 'Windows' in platform.platform():
        mm = 'target\debug\makespan-minimization.exe '
    else:
        mm = 'target/debug/makespan-minimization '

    for r in range(runs):
        i = 0
        start_time = time.time()
        s = time.strftime("%Y-%m-%d_%H-%M-%S", time.localtime())

        for file in os.listdir("./benchmarks"):
            if os.path.isfile('./benchmarks/' + file) and ".txt" in file:
                print(f"{i}.: starting with input: '" + file + "'")

                os.system(
                    f"{mm}--path ./benchmarks/{file} {args} --measurement >> logs_{s}.txt")  # --write --write-separate-files

                i += 1
                print("end with input: '" + file + "' -----------------------\n")

        end_time = time.time()
        print(f"time: {end_time - start_time} sec")
        logs = open(f"logs_{s}.txt", 'a')
        logs.write(f"\ntime: {end_time - start_time} sec\n")
        logs.write(f"\nFRAMEWORK_CONFIG: {args}\n")
        logs.write(f"\nNAME: {name}_{r}\n")
        print(f"generated logs are written in logs_{s}.txt")


# Create and launch a thread
t = Thread(target=run_all, args=())
t.start()
print("run-all is now running in the background")
