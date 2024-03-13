import platform
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

    files_subset = [
        "p_cmax-n10-m2-minsize100-maxsize800-seed5104.txt",
        "p_cmax-n11-m3-minsize100-maxsize200-seed4462.txt",
        "p_cmax-n13-m3-minsize100-maxsize200-seed694.txt",
        "p_cmax-n13-m3-minsize100-maxsize200-seed20154.txt",
        "p_cmax-n14-m3-minsize1-maxsize100-seed15146.txt",
        "p_cmax-n14-m3-minsize1-maxsize100-seed15994.txt",
        "p_cmax-n14-m3-minsize100-maxsize200-seed30986.txt",
        "p_cmax-n16-m3-minsize1-maxsize100-seed28095.txt",
        "p_cmax-n16-m3-minsize100-maxsize200-seed11945.txt",
        "p_cmax-n16-m5-minsize100-maxsize200-seed25087.txt",
        "p_cmax-n17-m3-minsize100-maxsize200-seed10300.txt",
        "p_cmax-n20-m4-minsize1-maxsize20-seed14287.txt",
        "p_cmax-n30-m3-minsize100-maxsize800-seed20478.txt",
        "p_cmax-n31-m10-minsize1-maxsize100-seed23229.txt",
        "p_cmax-n31-m10-minsize100-maxsize200-seed15534.txt",
        "p_cmax-n32-m10-minsize1-maxsize100-seed21779.txt",
        "p_cmax-n100-m6-minsize100-maxsize800-seed32372.txt",
        "p_cmax-n100-m10-minsize100-maxsize800-seed21386.txt"]

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

                                        start_time = time.time()

                                        s = time.strftime("%Y-%m-%d_%H-%M-%S", time.localtime())
                                        logs = open(f"logsHP/logs_{s}.txt", 'a')
                                        logs.write(f"\nFRAMEWORK_CONFIG: {args}\n")
                                        logs.write(f"NAME: {name}\n")
                                        logs.write(f"{s}\n")
                                        logs.close()
                                        logs = open(f"logsHP/logs_{s}.txt", 'a')

                                        for file in files_subset:
                                            print(f"{i}.: starting with input: '" + file + "'")

                                            # os.system(f"{mm}--path ./benchmarks/{file} {args} --measurement >> logsHP/logs_{s}.txt 2>&1 &")  # --write --write-separate-files DAS BRAUCHT MAN FÜR WINDOWS!
                                            subprocess.run([f"./{mm}--path ./benchmarks/{file} {args} --measurement"],
                                                           stdout=logs,
                                                           stderr=logs, shell=True)

                                            i += 1
                                            print("end with input: '" + file + "' -----------------------\n")

                                        end_time = time.time()
                                        logs.write(f"\ntime: {end_time - start_time} sec\n")
                                        print(f"generated logs are written in logsHP/logs_{s}.txt")
                                        print(f"time: {end_time - start_time} sec")


run_all()
