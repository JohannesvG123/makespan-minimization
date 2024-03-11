import os
import re
from dataclasses import dataclass


def run():
    @dataclass
    class Measurement:
        log_file_name: str
        instance_count: int
        runtime_sum_overall: float
        opt_found_count: int
        all_algos_finished_count: int
        timeout_count: int
        upper_bounds: list[([float], [int])]  # upper_bounds[instance_index]=(list of time,list of new bound)
        lower_bounds: list[([float], [int])]
        config: str
        instance_file_names: list[str]
        runtimes_per_instance: list[float]
        opt_found_per_instance: list[bool]
        config_name: str

    measurements: [Measurement] = []

    for file in os.listdir():
        if ".txt" in file and "logs" in file:
            print(f"reading logfile: {file}")
            log = open(file, 'r')
            lines = log.readlines()

            instances = 0
            instance_names = []
            runtime: float = 0
            opt_found = 0
            all_algos_finished = 0
            timeout = 0
            upper_bounds: list[([float], [int])] = []  # upper_bounds[instance_index]=(list of time,list of new bound)
            lower_bounds: list[([float], [int])] = []
            config = ""
            runtimes = []
            opt_found_per_instance = []
            name = ""

            between_start_end = False  # for the case when opt is found aut t=timeout_time (then it can happen, that two END messages are printed)
            for line in lines:
                if "start with input" in line:
                    instances += 1
                    instance_names.append(line.split("\"")[1])
                    between_start_end = True
                if "END" in line:
                    # extract time:
                    x = re.findall(" \d+\.\d+", line)
                    x = re.findall("[^\s-]*", x[0])
                    t = float(x[1])
                    runtime += t
                    runtimes.append(t)
                    if not between_start_end:
                        print("ACHTUNG das sollte seit commit \"bugfix (duplicated END)\" nicht mehr passieren!", t)
                    between_start_end = False
                    if "found OPT solution" in line:
                        opt_found += 1
                        opt_found_per_instance.append(True)
                    if "all algorithms finished" in line:
                        all_algos_finished += 1
                        opt_found_per_instance.append(False)
                    if "timeout" in line:
                        timeout += 1
                        opt_found_per_instance.append(False)
                if "trivial bounds:" in line:
                    ub = int((line.split('UB:')[1]).split(' ')[0])
                    lb = int((line.split('LB:')[1]).split(' ')[0])
                    upper_bounds.append(([0], [ub]))
                    lower_bounds.append(([0], [lb]))
                if "NEW upper_bound" in line:
                    time = float(line.split(' ')[4])
                    ub = int((line.split('->')[1]).split(' ')[0])
                    (upper_bounds[instances - 1][0]).append(time)
                    (upper_bounds[instances - 1][1]).append(ub)
                if "NEW lower_bound" in line:
                    time = float(line.split(' ')[4])
                    lb = int((line.split('->')[1]).split(' ')[0])
                    (lower_bounds[instances - 1][0]).append(time)
                    (lower_bounds[instances - 1][1]).append(lb)
                if "FRAMEWORK_CONFIG:" in line:
                    config = line.split('FRAMEWORK_CONFIG:')[1].strip()
                if "NAME: " in line:
                    name = line.split('NAME:')[1].strip()

            measurements.append(
                Measurement(file, instances, runtime, opt_found, all_algos_finished, timeout, upper_bounds,
                            lower_bounds, config, instance_names, runtimes, opt_found_per_instance, name))
    return measurements


print(run())
