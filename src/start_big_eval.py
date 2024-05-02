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

for threads in [1, 2, 4, 8, 16, 32]:
    processes = []  # todo immer 32 threads nutzen
    files_subsets = split_list_into_sublists(files, int(32 / threads))
    for i in range(files_subsets.__len__()):
        print("leggo", threads, i)
        # run_all(files_subsets[i], i,threads)
        p = subprocess.Popen(["python3", "-c",
                              f"from run_all_parameterized_long import run_all; run_all({files_subsets[i]}, {i},{threads})"])
        processes.append(p)

    for p in processes:
        p.wait()

    print("FINISHED ", threads, " :)")
