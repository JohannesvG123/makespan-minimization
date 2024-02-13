import os
import re


def run():
    measurements = []

    for file in os.listdir():
        if ".txt" in file and "logs" in file:
            print(f"reading logfile: {file}")
            log = open(file, 'r')
            lines = log.readlines()

            instances = 0
            runtime: float = 0
            opt_found = 0
            all_algos_finished = 0
            timeout = 0
            upper_bounds: list[([float], [int])] = []  # upper_bounds[instance_index]=(list of time,list of new bound)
            lower_bounds: list[([float], [int])] = []

            for line in lines:
                if "start with input" in line:
                    instances += 1
                if "END" in line:
                    # extract time:
                    x = re.findall(" \d+\.\d+", line)
                    x = re.findall("[^\s-]*", x[0])
                    t = float(x[1])
                    runtime += t
                    if "found OPT solution" in line:
                        opt_found += 1
                    if "all algorithms finished" in line:
                        all_algos_finished += 1
                    if "timeout" in line:
                        timeout += 1
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

            measurements.append(
                (file, instances, runtime, opt_found, all_algos_finished, timeout, upper_bounds, lower_bounds))
    return measurements


print(run())