import os
import platform
import random
import subprocess
import time


# working directory must be "/makespan-minimization"!

# run config on all instances (==.txt files) in benchmarks directory
# example config: "--bf --lpt --rr --rf --swap --ff --rf-configs , --swap-configs ,,"

def find_indices(search_list, search_item):
    return [index for (index, item) in enumerate(search_list) if search_item in item]


def run_all():  # FOR HYPERPARAMETER TUNING
    # os.system('cargo build')
    mm = ''
    if 'Windows' in platform.platform():
        mm = 'target\debug\makespan-minimization.exe '
    else:
        mm = 'target/debug/makespan-minimization '

    files = os.listdir("./benchmarks")
    files_wo_seed = []
    files_subset = []  # wird am Ende 2 instanzen jeder größer enthalten (mit 2 verschiedenen seeds):
    for file in files:
        if not (os.path.isfile('./benchmarks/' + file) and ".txt" in file):
            files.remove(file)
        else:
            s = file.split("-seed")[0]
            if not files_wo_seed.__contains__(s):
                files_wo_seed.append(s)
    for i in range(files_wo_seed.__len__()):
        indices = find_indices(files, files_wo_seed[i])
        i1 = indices.pop(random.randint(0, indices.__len__() - 1))
        files_subset.append(files[i1])
        # if indices.__len__() > 0: #AUSKOMMENTIEREN FÜR GRÖSSERES TESTSET
        #     i2 = indices.pop(random.randint(0, indices.__len__() - 1))
        #     files_subset.append(files[i2])

    os.system(f"{files_subset} >> logsHPnew/instances.txt")
    inst = open(f"logsHPnew/instances.txt", 'a')
    inst.write(f"{files_subset.__len__()}{files_subset}\n")
    inst.close()

    skip = True
    for do_restart_after_steps in [True, False]:
        for restart_after_steps in [5, 10, 25, 50, 100, 200, 300]:
            for random_restart_possibility in [0.0, 0.25, 0.5, 0.75, 1.0]:
                for restart_scaling_factor in [1.0, 1.2, 2]:
                    for l in [0.1, 0.5, 5.0]:
                        for timeout_after in [200]:
                            for num_solutions in [5000]:
                                configs = []
                                if do_restart_after_steps:
                                    tmp = f"true,{restart_after_steps},,{restart_scaling_factor},{random_restart_possibility},{l}"
                                    configs = [
                                        f"two-job-random-swap,all,1,{tmp} two-job-random-swap,decline-by-5%-chance,1,{tmp} two-job-random-swap,decline-by-20%-chance,1,{tmp} two-job-random-swap,decline-by-50%-chance,1,{tmp} two-job-brute-force,improvement,1,{tmp} two-job-brute-force,decline-by-5%-chance,1,{tmp} two-job-brute-force,decline-by-20%-chance,1,{tmp} two-job-brute-force,decline-by-50%-chance,1,{tmp}",
                                        f",,max,{tmp}"]
                                else:
                                    tmp = f"false,,{restart_after_steps * 0.001},{restart_scaling_factor},{random_restart_possibility},{l}"
                                    configs = [
                                        f"two-job-random-swap,all,1,{tmp} two-job-random-swap,decline-by-5%-chance,1,{tmp} two-job-random-swap,decline-by-20%-chance,1,{tmp} two-job-random-swap,decline-by-50%-chance,1,{tmp} two-job-brute-force,improvement,1,{tmp} two-job-brute-force,decline-by-5%-chance,1,{tmp} two-job-brute-force,decline-by-20%-chance,1,{tmp} two-job-brute-force,decline-by-50%-chance,1,{tmp}",
                                        f",,max,{tmp}"]
                                for config in configs:
                                    for num_threads in [8]:
                                        i = 0
                                        args = f"--num-threads {num_threads} --num-solutions {num_solutions} --timeout-after {timeout_after} --swap-configs {config} --bf --ff --lpt --rr --swap --rf --rf-configs , , , ,"
                                        name = f"HYPERP-{num_threads}-threads,{num_solutions}-sol,{timeout_after}s-timeout,{config}"

                                        if name == "HYPERP-8-threads,5000-sol,200s-timeout,,,max,true,25,,2,0.25,5.0":
                                            skip = False

                                        if not skip:
                                            start_time = time.time()

                                            s = time.strftime("%Y-%m-%d_%H-%M-%S", time.localtime())
                                            logs = open(f"logsHPnew/logs_{s}.txt", 'a')
                                            logs.write(f"\nFRAMEWORK_CONFIG: {args}\n")
                                            logs.write(f"NAME: {name}\n")
                                            logs.write(f"{s}\n")
                                            logs.close()
                                            logs = open(f"logsHPnew/logs_{s}.txt", 'a')

                                            for file in files_subset:
                                                print(f"{i}.: starting with input: '" + file + "'")

                                                # os.system(f"{mm}--path ./benchmarks/{file} {args} --measurement >> logsHPnew/logs_{s}.txt 2>&1 &")  # --write --write-separate-files DAS BRAUCHT MAN FÜR WINDOWS!
                                                subprocess.run(
                                                    [f"./{mm}--path ./benchmarks/{file} {args} --measurement"],
                                                    stdout=logs,
                                                    stderr=logs, shell=True)

                                                i += 1
                                                print("end with input: '" + file + "' -----------------------\n")

                                            end_time = time.time()
                                            logs.write(f"\ntime: {end_time - start_time} sec\n")
                                            print(f"generated logs are written in logsHPnew/logs_{s}.txt")
                                            print(f"time: {end_time - start_time} sec")


run_all()
