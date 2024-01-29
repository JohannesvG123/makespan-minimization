import os

# working directory must be "/makespan-minimization"!

# Validate all solutions in data/...solution...

for dir in os.listdir("./data"):
    if os.path.isdir("./data/" + dir) and "solution" in dir:
        print("VALIDATE: ./data/" + dir + "----------------------------------------- \n")

        input_file = './benchmarks/' + dir.split('_solution').pop(0) + '.txt'

        os.system('py data/validate-multiple-solutions-all.py ' + input_file + " ./data/" + dir)

        print("\n")
