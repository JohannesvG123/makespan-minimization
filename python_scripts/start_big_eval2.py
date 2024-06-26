import os
import subprocess


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


files = os.listdir("./benchmarks/all_benchmarks_with_opt")
for config in ["two-job-random-swap,improvement,max",
               "two-job-best-swap,improvement,8 two-job-best-swap,improvement-or-rs-by-5%-chance,8",
               "two-job-best-swap,improvement,4 two-job-best-swap,improvement-or-rs-by-5%-chance,4 two-job-random-swap,improvement,4 two-job-random-swap,decline-by-5%-chance,4",
               "two-job-best-swap,improvement-or-rs-by-5%-chance,8 two-job-random-swap,improvement,8",
               "two-job-best-swap,improvement,4 two-job-best-swap,improvement-or-rs-by-5%-chance,8 two-job-random-swap,improvement,2 two-job-random-swap,decline-by-5%-chance,2"]:
    for threads in [8,16,32]:
        if (config == "two-job-random-swap,improvement,max" and threads == 8) or config == "two-job-random-swap,decline-by-5%-chance,max":
            processes = []
            files_subsets = split_list_into_sublists(files, int(64 / threads))
            for i in range(files_subsets.__len__()):
                print("leggo", threads, i)
                # run_all(files_subsets[i], i,threads)
                with open(f"tmp3_{threads}_{i}.txt", 'w') as f:
                    for file in files_subsets[i]:
                        f.write(file + '\n')
                p = subprocess.Popen(["python3", "-c",
                                      f"from src.run_all_parameterized_long2 import run_all; run_all({[f'tmp3_{threads}_{i}.txt']}, {i},{threads},\"{config}\")"])
                processes.append(p)

            for p in processes:
                p.wait()

            print("FINISHED ", threads, " :)\n")
