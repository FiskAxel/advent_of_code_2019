fn main() {
    let input = std::fs::read_to_string("src/input01.txt")
        .expect("Unreadable");
    let nums: Vec<i32> = input
        .trim()
        .lines()
        .map(|n| n.parse::<i32>().expect("parse error"))
        .collect();

    println!("Part 1: {}", part1(&nums));
    println!("Part 2: {}", part2(&nums));
}

fn part1(nums: &Vec<i32>) -> i32 {
    let mut sum: i32 = 0;
    for num in nums.iter() {
        sum += calculate_fuel(num);
    }
    return sum;
}

fn part2(nums: &Vec<i32>) -> i32 {
    let mut sum: i32 = 0;
    for num in nums.iter() {
        sum += calculate_fuel_recursive(num);
    }
    return sum;
}

fn calculate_fuel(num: &i32) -> i32 {
    return (num / 3) - 2;
}

fn calculate_fuel_recursive(num: &i32) -> i32 {
    let mut sum: i32 = 0;
    let fuel_required = calculate_fuel(&num);
    if fuel_required <= 0 {
        return 0;
    }
    sum += fuel_required + calculate_fuel_recursive(&fuel_required);
    return sum;
}
