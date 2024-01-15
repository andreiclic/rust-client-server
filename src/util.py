import sys


def extract_response_times(file_path):
    response_times = []

    with open(file_path, "r") as f:
        for line in f.readlines():
            if "The previous request took" in line:
                response_times.append(int(line.split()[-1][:-2]))

    return response_times


def main():
    response_times = extract_response_times(sys.argv[1])


if __name__ == "__main__":
    main()
