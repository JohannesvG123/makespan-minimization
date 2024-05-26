import glob
import os
import shutil


def move_files_by_datetime(directory):
    # Iterate over all files in the directory
    for filename in os.listdir(directory):
        # Split the filename into parts based on underscores
        parts = filename.split('_')
        # Extract the date (d) and timestamp (t) parts
        date_part = parts[1]
        time_part = parts[2]
        c = ''
        t = ''
        with open(directory + '/' + filename, 'r') as file:
            # print(file.readlines()[2])
            parts = file.readlines()[2].split(',')
            c = parts[3] + "_" + parts[4]
            t = parts[0].split('-')[-2]

        # Form the new directory name
        new_directory = os.path.join(directory, f"logs_{c}_{t}-threads")

        # Create the new directory if it doesn't exist
        os.makedirs(new_directory, exist_ok=True)

        # Move the file to the new directory
        shutil.move(os.path.join(directory, filename), os.path.join(new_directory, filename))


def merge_files(directory):
    # Get all txt files in the directory
    txt_files = glob.glob(os.path.join(directory, '*.txt'))

    # Open the output file in append mode
    with open(f"{directory}.txt", 'a') as output_file:
        # Loop through each txt file
        fr = True
        for file_path in txt_files:
            with open(file_path, 'r') as file:
                lines = file.readlines()

                if not fr:
                    # Remove first 4 lines and last 2 lines
                    trimmed_lines = lines[4:-2]
                else:
                    trimmed_lines = lines[0:-2]

                # Write the trimmed lines to the output file
                output_file.writelines(trimmed_lines)

                # Add a newline after each file
                output_file.write('\n')

                fr = False


# Specify the directory containing the files
# directory = '.'

# Call the function to move files based on date and timestamp
# move_files_by_datetime(directory)

for dir in os.listdir("../src"):
    print(dir)
    merge_files("./" + dir)

'''
import os
import re


def remove_lines_containing_error(input_file, output_file):
    # Open the input file for reading
    with open(input_file, 'r') as input_file_handle:
        # Open the output file for writing
        with open(output_file, 'w') as output_file_handle:
            # Read the input file line by line
            for line in input_file_handle:
                # Check if the line contains "Error: reached"
                if not re.search(r'Error: reached', line):
                    # Write the line to the output file if it doesn't contain the pattern
                    output_file_handle.write(line)


def process_files_in_directory(directory):
    # Iterate over all files in the directory
    for filename in os.listdir(directory):
        input_file = os.path.join(directory, filename)
        # Check if the file is a text file
        filename.__contains__()
        if filename.endswith('.txt'):
            output_file = input_file + '.tmp'
            # Remove lines containing "Error: reached" from the input file and write to the output file
            remove_lines_containing_error(input_file, output_file)
            # Replace the original file with the updated one
            os.replace(output_file, input_file)


for dir in os.listdir():
    if not dir.endswith(".txt"):
        print(dir)
        # Call the function to remove lines containing "Error: reached" from all text files in the directory
        process_files_in_directory(dir)
'''
'''
optfile = open(f"./benchmarks/more/opt-known-instances-franca.txt", 'r')
lines = optfile.readlines()
print(len(lines))

for line in lines:
    print(line.split())
    instance = line.split()[0]
    opt = int(line.split()[1])
    instancefile = open(f"./benchmarks/more/franca_frangioni/standardised/{instance}",'a')
    instancefile.write(f"\nOPT:{opt};")

i=0
for file in os.listdir(f"./benchmarks/all_benchmarks_with_opt/"):
    instancefile = open(f"./benchmarks/all_benchmarks_with_opt/{file}", 'r')
    if not instancefile.readlines()[2].__contains__("OPT"):
        print("ALARM: ", file)
        i+=1

print(i)

'''
'''
def remove_lines_around_pattern(file_path, pattern, lines_before, lines_after):
    with open(file_path, 'r') as file:
        lines = file.readlines()

    with open(file_path, 'w') as file:
        skip_lines = 0
        for i, line in enumerate(lines):
            if skip_lines > 0:
                skip_lines -= 1
                continue

            if pattern in line:
                start_index = max(0, i - lines_before)
                end_index = min(len(lines), i + lines_after + 1)
                del lines[start_index:end_index]
                skip_lines = lines_after

        file.writelines(lines)

# Example usage:
file_path = "log_archive/logs_2024-04-23_21-22-29.txt"  # Replace 'example.txt' with the path to your text file
instancefiles = os.listdir(f"./benchmarks/all_benchmarks_with_opt/")
pattern = "panic"
lines_before = 1  # Number of lines before the pattern to remove
lines_after = 2   # Number of lines after the pattern to remove
remove_lines_around_pattern(file_path, pattern, lines_before, lines_after)
'''

'''
def filter_lines(input_file, instance_files):
    result_lines = []
    with open(input_file, 'r') as file:
        in_file = None
        keep_lines = True
        for line in file:
            if "start with input" in line:
                keep_lines = True
                in_file = line.split("\"")[1].split("/")[3]
                if in_file is not None and in_file not in instance_files:
                    keep_lines = False
            if keep_lines:
                result_lines.append(line)
    return result_lines


# Example usage:
input_file = "log_archive/1_ALL_BENCHMARKS_pc135_speedrun/logs_2024-04-27_01-43-52.txt"
instance_files = os.listdir(f"./benchmarks/all_benchmarks_with_opt/")
filtered_lines = filter_lines(input_file, instance_files)

# Write the filtered lines to a new file
with open("log_archive/1_ALL_BENCHMARKS_pc135_speedrun/logs_2024-04-27_01-43-52-better.txt", 'w') as output_file:
    output_file.writelines(filtered_lines)
'''
