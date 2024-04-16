import os
import platform
import subprocess
import time


# working directory must be "/makespan-minimization"!

# run config on all instances (==.txt files) in benchmarks directory
# example config: "--bf --lpt --rr --rf --swap --ff --rf-configs , --swap-configs ,,"

def find_indices(search_list, search_item):
    return [index for (index, item) in enumerate(search_list) if search_item in item]


def run_all():
    os.system('cargo build')
    mm = ''
    if 'Windows' in platform.platform():
        mm = 'target\debug\makespan-minimization.exe '
    else:
        mm = 'target/debug/makespan-minimization '

    files = os.listdir("./benchmarks")

    configs = ["two-job-brute-force,improvement,max"]

    for timeout_after in [300]:
        for num_solutions in [500]:
            for config in configs:
                for num_threads in [1, 2, 4, 8, 16, 32, 64, 128]:
                    i = 0
                    if config != "two-job-random-swap,decline-by-20%-chance,1 two-job-random-swap,decline-by-5%-chance,1 two-job-random-swap,improvement,1 two-job-random-swap,all,1 two-job-brute-force,improvement,1 two-job-brute-force,improvement-or-rs-by-5%-chance,1 two-job-brute-force,improvement-or-rs-by-20%-chance,1 two-job-brute-force,improvement-or-rs-by-50%-chance,1" or num_threads == 8:
                        args = f"--num-threads {num_threads} --num-solutions {num_solutions} --timeout-after {timeout_after} --swap-configs {config} --bf --ff --lpt --rr --swap --rf --rf-configs , , , ,"
                        name = f"allInstances-{num_threads}-threads,{num_solutions}-sol,{timeout_after}s-timeout,{config}"

                        prog = open(f"logs/progress.txt", 'a')
                        prog.write(f"running: {name}")
                        prog.close()

                        start_time = time.time()

                        s = time.strftime("%Y-%m-%d_%H-%M-%S", time.localtime())
                        logs = open(f"logs/logs_{s}.txt", 'a')
                        logs.write(f"\nFRAMEWORK_CONFIG: {args}\n")
                        logs.write(f"NAME: {name}\n")
                        logs.write(f"{s}\n")
                        logs.close()
                        logs = open(f"logs/logs_{s}.txt", 'a')

                        for file in files:  # or files_subset
                            if ".txt" in file:
                                print(f"{i}.: starting with input: '" + file + "'")

                                # os.system(f"{mm}--path ./benchmarks/{file} {args} --measurement >> logs/logs_{s}.txt 2>&1 &")  # --write --write-separate-files DAS BRAUCHT MAN FÜR WINDOWS!
                                subprocess.run([f"./{mm}--path ./benchmarks/{file} {args} --measurement"], stdout=logs,
                                               stderr=logs, shell=True)

                                i += 1
                                print("end with input: '" + file + "' -----------------------\n")

                        end_time = time.time()
                        logs.write(f"\ntime: {end_time - start_time} sec\n")
                        print(f"generated logs are written in logs/logs_{s}.txt")
                        print(f"time: {end_time - start_time} sec")

                        prog = open(f"logs/progress.txt", 'a')
                        prog.write(f"\nfinished.\n")
                        prog.close()


run_all()
