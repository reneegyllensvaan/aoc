import re
import sys
from time import time


def part1_renee_port(input: str) -> int:
    result = 0;
    for line in input.split("\n"):
        first = None
        last = None
        for c in line:
            try:
                digit = int(c)
            except ValueError:
                continue
            if first is None:
                first = digit
            last = digit
        if first is not None and last is not None:
            result += first*10 + last
    return result

NUMBERS = {
    "one": "1",
    "two": "2",
    "three": "3",
    "four": "4",
    "five": "5",
    "six": "6",
    "seven": "7",
    "eight": "8",
    "nine": "9",
}
def part2_j(input):
    tot = 0
    for line in input.splitlines():
        first = ""
        last_candidate = ""
        for idx, char in enumerate(line):
            if char.isdigit():
                if not first:
                    first = char
                last_candidate = char
                continue
            end_string_idx = idx + 2
            while end_string_idx < len(line):
                substring = line[idx:end_string_idx]
                if substring in NUMBERS:
                    if not first:
                        first = NUMBERS[substring]
                    last_candidate = NUMBERS[substring]
                end_string_idx += 1

        tot += int(first + last_candidate)
    return tot

def part2_renee_regex_port(input):
    result = 0
    exp = re.compile(r"^([0-9]|one|two|three|four|five|six|seven|eight|nine|.*?)");
    lut = {
        "1": 1,
        "2": 2,
        "3": 3,
        "4": 4,
        "5": 5,
        "6": 6,
        "7": 7,
        "8": 8,
        "9": 9,

        "one": 1,
        "two": 2,
        "three": 3,
        "four": 4,
        "five": 5,
        "six": 6,
        "seven": 7,
        "eight": 8,
        "nine": 9,
    }
    for line in input.splitlines():
        first = None
        last = None
        end = 0
        while end < len(line):
            tail = line[end:]
            m = exp.match(tail)
            end += 1;
            g = m.group() if m is not None else None
            if not g:
                continue
            digit = lut.get(g)

            if first is None:
                first = digit
            last = digit
        if first is not None and last is not None:
            result += first*10 + last
    return result

with open("input/day01", "r") as f:
    data = f.read()

def time_it(f, times=1000):
    begin = time()
    for _ in range(times):
        f(data)
    end = time()

    print(f"{times} {f.__name__} in: {int((end-begin) * 1000000)}us ({int(((end-begin) * 1000000) / times)}us/iter)")

fns = [
    part1_renee_port,
    part2_j,
    part2_renee_regex_port,
]

for f in fns:
    print(f"{f.__name__}: {f(data)}")
print("")
for f in fns:
    time_it(f, 100)
