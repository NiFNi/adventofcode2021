def main():
    with open('input') as f:
        line = f.readline()
    values = [int(a) for a in line.split(",")]
    max_value = max(values)
    print(min([calculate_fuel(values, a, calculate_fuel_part1) for a in range(0, max_value)]))
    print(min([calculate_fuel(values, a, calculate_fuel_part2) for a in range(0, max_value)]))


def calculate_fuel(values, target, f):
    return sum([f(target, a) for a in values])


def calculate_fuel_part1(target, position):
    return abs(position - target)


def calculate_fuel_part2(target, position):
    n = calculate_fuel_part1(target, position)
    return int(0.5 * n * (n + 1))


if __name__ == '__main__':
    main()
