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
    files_wo_seed = []
    files_subset = ['p_cmax-n31-m10-minsize1-maxsize100-seed26011.txt',
                    'p_cmax-n26-m8-minsize100-maxsize200-seed27674.txt',
                    'p_cmax-n100-m4-minsize100-maxsize800-seed26520.txt',
                    'p_cmax-n20-m4-minsize20-maxsize50-seed3070.txt', 'p_cmax-n17-m3-minsize1-maxsize100-seed30661.txt',
                    'p_cmax-n33-m8-minsize1-maxsize100-seed22400.txt',
                    'p_cmax-n33-m8-minsize100-maxsize200-seed21977.txt',
                    'p_cmax-n12-m4-minsize20-maxsize50-seed5280.txt',
                    'p_cmax-n9-m3-minsize100-maxsize200-seed10200.txt',
                    'p_cmax-n8-m4-minsize20-maxsize50-seed28213.txt', 'p_cmax-n14-m3-minsize1-maxsize100-seed2844.txt',
                    'p_cmax-n10-m2-minsize100-maxsize800-seed16609.txt',
                    'p_cmax-n10-m2-minsize100-maxsize200-seed29636.txt',
                    'p_cmax-n17-m5-minsize100-maxsize200-seed2840.txt', 'p_cmax-n9-m3-minsize20-maxsize50-seed7610.txt',
                    'p_cmax-n15-m3-minsize20-maxsize50-seed3217.txt',
                    'p_cmax-n16-m5-minsize100-maxsize200-seed9426.txt',
                    'p_cmax-n100-m3-minsize100-maxsize800-seed26279.txt',
                    'p_cmax-n17-m5-minsize1-maxsize100-seed19957.txt',
                    'p_cmax-n11-m3-minsize100-maxsize200-seed12544.txt',
                    'p_cmax-n30-m10-minsize100-maxsize800-seed13680.txt',
                    'p_cmax-n10-m3-minsize1-maxsize100-seed13295.txt',
                    'p_cmax-n16-m3-minsize1-maxsize100-seed13748.txt', 'p_cmax-n10-m2-minsize1-maxsize20-seed1948.txt',
                    'p_cmax-n100-m10-minsize100-maxsize800-seed6592.txt',
                    'p_cmax-n41-m8-minsize100-maxsize200-seed20415.txt',
                    'p_cmax-n50-m4-minsize100-maxsize800-seed10439.txt',
                    'p_cmax-n10-m5-minsize1-maxsize20-seed17277.txt', 'p_cmax-n26-m8-minsize1-maxsize100-seed22806.txt',
                    'p_cmax-n50-m3-minsize100-maxsize800-seed7646.txt',
                    'p_cmax-n10-m2-minsize1-maxsize100-seed3714.txt', 'p_cmax-n9-m3-minsize1-maxsize100-seed4527.txt',
                    'p_cmax-n27-m5-minsize1-maxsize100-seed960.txt',
                    'p_cmax-n50-m6-minsize100-maxsize800-seed26031.txt',
                    'p_cmax-n22-m5-minsize100-maxsize200-seed9067.txt',
                    'p_cmax-n10-m3-minsize100-maxsize800-seed4209.txt',
                    'p_cmax-n50-m10-minsize100-maxsize800-seed4718.txt',
                    'p_cmax-n26-m5-minsize1-maxsize100-seed21400.txt', 'p_cmax-n42-m8-minsize1-maxsize100-seed8038.txt',
                    'p_cmax-n14-m3-minsize100-maxsize200-seed10311.txt',
                    'p_cmax-n32-m10-minsize1-maxsize100-seed2407.txt',
                    'p_cmax-n30-m2-minsize100-maxsize800-seed14213.txt',
                    'p_cmax-n6-m3-minsize20-maxsize50-seed4353.txt', 'p_cmax-n6-m3-minsize1-maxsize20-seed7474.txt',
                    'p_cmax-n42-m8-minsize100-maxsize200-seed18349.txt',
                    'p_cmax-n16-m3-minsize100-maxsize200-seed3911.txt',
                    'p_cmax-n100-m8-minsize100-maxsize800-seed17408.txt',
                    'p_cmax-n27-m5-minsize100-maxsize200-seed16014.txt',
                    'p_cmax-n50-m2-minsize100-maxsize800-seed1042.txt',
                    'p_cmax-n15-m3-minsize1-maxsize20-seed29765.txt', 'p_cmax-n16-m5-minsize1-maxsize100-seed25297.txt',
                    'p_cmax-n15-m5-minsize20-maxsize50-seed26855.txt',
                    'p_cmax-n21-m5-minsize1-maxsize100-seed16055.txt',
                    'p_cmax-n42-m10-minsize1-maxsize100-seed17206.txt',
                    'p_cmax-n10-m3-minsize100-maxsize200-seed7833.txt',
                    'p_cmax-n25-m5-minsize20-maxsize50-seed7018.txt',
                    'p_cmax-n17-m3-minsize100-maxsize200-seed12420.txt',
                    'p_cmax-n34-m8-minsize1-maxsize100-seed1279.txt',
                    'p_cmax-n31-m10-minsize100-maxsize200-seed29544.txt',
                    'p_cmax-n52-m10-minsize100-maxsize200-seed25845.txt',
                    'p_cmax-n9-m3-minsize100-maxsize800-seed25068.txt',
                    'p_cmax-n15-m5-minsize1-maxsize20-seed27949.txt',
                    'p_cmax-n32-m10-minsize100-maxsize200-seed25378.txt',
                    'p_cmax-n30-m4-minsize100-maxsize800-seed21407.txt',
                    'p_cmax-n30-m8-minsize100-maxsize800-seed4674.txt',
                    'p_cmax-n42-m10-minsize100-maxsize200-seed32756.txt',
                    'p_cmax-n50-m8-minsize100-maxsize800-seed13843.txt',
                    'p_cmax-n20-m4-minsize1-maxsize20-seed25886.txt', 'p_cmax-n9-m3-minsize1-maxsize20-seed15462.txt',
                    'p_cmax-n30-m3-minsize100-maxsize800-seed10751.txt',
                    'p_cmax-n51-m10-minsize1-maxsize100-seed25144.txt',
                    'p_cmax-n11-m3-minsize1-maxsize100-seed11056.txt',
                    'p_cmax-n10-m5-minsize20-maxsize50-seed29904.txt',
                    'p_cmax-n21-m5-minsize100-maxsize200-seed23813.txt',
                    'p_cmax-n25-m8-minsize100-maxsize200-seed6183.txt',
                    'p_cmax-n26-m5-minsize100-maxsize200-seed29815.txt',
                    'p_cmax-n13-m3-minsize1-maxsize100-seed26153.txt',
                    'p_cmax-n22-m5-minsize1-maxsize100-seed24011.txt', 'p_cmax-n10-m2-minsize20-maxsize50-seed44.txt',
                    'p_cmax-n30-m6-minsize100-maxsize800-seed7915.txt', 'p_cmax-n25-m5-minsize1-maxsize20-seed4239.txt',
                    'p_cmax-n100-m2-minsize100-maxsize800-seed13907.txt',
                    'p_cmax-n10-m2-minsize50-maxsize100-seed10596.txt',
                    'p_cmax-n52-m10-minsize1-maxsize100-seed26710.txt',
                    'p_cmax-n34-m8-minsize100-maxsize200-seed22000.txt',
                    'p_cmax-n41-m8-minsize1-maxsize100-seed4231.txt', 'p_cmax-n12-m4-minsize1-maxsize20-seed14888.txt',
                    'p_cmax-n41-m10-minsize1-maxsize100-seed16514.txt',
                    'p_cmax-n41-m10-minsize100-maxsize200-seed13556.txt',
                    'p_cmax-n8-m4-minsize1-maxsize20-seed19891.txt', 'p_cmax-n25-m8-minsize1-maxsize100-seed22694.txt',
                    'p_cmax-n9-m3-minsize50-maxsize100-seed29557.txt',
                    'p_cmax-n51-m10-minsize100-maxsize200-seed24531.txt',
                    'p_cmax-n100-m6-minsize100-maxsize800-seed20961.txt',
                    'p_cmax-n13-m3-minsize100-maxsize200-seed6592.txt']

    for timeout_after in [60]:
        for num_solutions in [500, 5000]:
            for num_threads in [8, 16]:
                i = 0
                config = "two-job-random-swap,all,1 two-job-random-swap,decline-by-5%-chance,1 two-job-random-swap,decline-by-20%-chance,1 two-job-random-swap,decline-by-50%-chance,1 two-job-brute-force,improvement,1 two-job-brute-force,decline-by-5%-chance,1 two-job-brute-force,decline-by-20%-chance,1 two-job-brute-force,decline-by-50%-chance,1"
                if num_threads == 16:
                    config = config + " " + config
                if num_threads == 32:
                    config = config + " " + config + " " + config + " " + config
                args = f"--num-threads {num_threads} --num-solutions {num_solutions} --timeout-after {timeout_after} --swap-configs {config} --bf --ff --lpt --rr --swap --rf --rf-configs , , , ,"
                name = f"{num_threads}-threads,{num_solutions}-sol,{timeout_after}s-timeout,mixed-config"

                start_time = time.time()

                s = time.strftime("%Y-%m-%d_%H-%M-%S", time.localtime())
                logs = open(f"logsM/logs_{s}.txt", 'a')
                logs.write(f"\nFRAMEWORK_CONFIG: {args}\n")
                logs.write(f"NAME: {name}\n")
                logs.write(f"{s}\n")
                logs.close()
                logs = open(f"logsM/logs_{s}.txt", 'a')

                for file in files_subset:
                    print(f"{i}.: starting with input: '" + file + "'")

                    # os.system(f"{mm}--path ./benchmarks/{file} {args} --measurement >> logsM/logs_{s}.txt 2>&1 &")  # --write --write-separate-files DAS BRAUCHT MAN FÜR WINDOWS!
                    subprocess.run([f"./{mm}--path ./benchmarks/{file} {args} --measurement"], stdout=logs,
                                   stderr=logs, shell=True)

                    i += 1
                    print("end with input: '" + file + "' -----------------------\n")

                end_time = time.time()
                logs.write(f"\ntime: {end_time - start_time} sec\n")
                print(f"generated logs are written in logsM/logs_{s}.txt")
                print(f"time: {end_time - start_time} sec")


run_all()
