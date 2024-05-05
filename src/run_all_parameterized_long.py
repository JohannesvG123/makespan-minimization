import platform
import subprocess

import time


# working directory must be "/makespan-minimization"!

# run config on all instances (==.txt files) in benchmarks directory
# example config: "--bf --lpt --rr --rf --swap --ff --rf-configs , --swap-configs ,,"

def find_indices(search_list, search_item):
    return [index for (index, item) in enumerate(search_list) if search_item in item]


def split_list_into_sublists(lst, x):
    num_elements_per_sublist = len(lst) // x
    remainder = len(lst) % x
    sublists = []

    for i in range(x):
        sublist = []
        for j in range(num_elements_per_sublist):
            index = j * x + i
            if index < len(lst):
                sublist.append(lst[index])
        if i < remainder:
            sublist.append(lst[num_elements_per_sublist * x + i])
        sublists.append(sublist)

    return sublists


def run_all(files, subset_i, num_threads_extern):
    if files[0].startswith("tmp_"):
        # get files from extra file because otherwise cmd args are too long
        with open(files[0], 'r') as f:
            files = [line.strip() for line in f]

    # os.system('cargo build')
    mm = ''
    if 'Windows' in platform.platform():
        mm = 'target\debug\makespan-minimization.exe '
    else:
        mm = 'target/debug/makespan-minimization '

    configs = ["two-job-best-swap,improvement-or-rs-by-5%-chance,max"]

    for timeout_after in [100]:
        for num_solutions in [500]:
            for config in configs:
                for num_threads in [num_threads_extern]:  # hier anpassen
                    i = 0
                    args = f"--num-threads {num_threads} --num-solutions {num_solutions} --timeout-after {timeout_after} --swap-configs {config} --bf --ff --lpt --rr --swap --rf --rf-configs , , , ,"
                    name = f"allInstancesp-{num_threads}-threads,{num_solutions}-sol,{timeout_after}s-timeout,{config}"

                    prog = open(f"logs/progress.txt", 'a')
                    prog.write(f"running: {name}")
                    prog.close()

                    start_time = time.time()

                    s = time.strftime("%Y-%m-%d_%H-%M-%S", time.localtime())
                    logs = open(f"logs/logs_{s}_{subset_i}.txt", 'a')
                    logs.write(f"\nFRAMEWORK_CONFIG: {args}\n")
                    logs.write(f"NAME: {name}\n")
                    logs.write(f"{s}\n")
                    logs.close()
                    logs = open(f"logs/logs_{s}_{subset_i}.txt", 'a')

                    for file in files:
                        if ".txt" in file:
                            print(f"{i}.: starting with input: '" + file + "'")

                            # os.system(f"{mm}--path ./benchmarks/{file} {args} --measurement >> logs/logs_{s}_{subset_i}.txt 2>&1 &")  # --write --write-separate-files DAS BRAUCHT MAN FÜR WINDOWS!
                            subprocess.run(
                                [f"./{mm}--path ./benchmarks/all_benchmarks_with_opt/{file} {args} --measurement"],
                                stdout=logs,
                                stderr=logs, shell=True)

                            i += 1
                            print("end with input: '" + file + "' -----------------------\n")

                    end_time = time.time()
                    logs.write(f"\ntime: {end_time - start_time} sec\n")
                    print(f"generated logs are written in logs/logs_{s}_{subset_i}.txt")
                    print(f"time: {end_time - start_time} sec")

                    prog = open(f"logs/progress.txt", 'a')
                    prog.write(f"\nfinished.\n")
                    prog.close()
