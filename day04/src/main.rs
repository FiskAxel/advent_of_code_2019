use std::collections::HashMap;
fn main() {
    let input = std::fs::read_to_string("src/input.txt").unwrap();
    let nums: Vec<i32> = input
        .split("-")
        .map(|n| n.parse::<i32>().expect("parse error"))
        .collect();

    let mut sum1 = 0;
    let mut sum2 = 0;
    for i in nums[0]..nums[1] {
        let s = i.to_string();
        
        if not_increasing(&s) {
            continue;
        } if no_double(&s) {
            continue;
        }
        sum1 += 1;

        if no_pair(&s) {
            continue
        }
        sum2 += 1;
    }
    println!("Part 1: {}", sum1);
    println!("Part 2: {}", sum2);
}

fn not_increasing(number: &String) -> bool {
    const RADIX: u32 = 10;
    let nums: Vec<u32> = number
        .chars()
        .map(|n| n.to_digit(RADIX).unwrap())
        .collect();

    for i in 1..nums.len() {
        if nums[i] < nums[i-1] {
            return true;
        }
    }
    return false;
}

fn no_double(number: &String) -> bool {
    let nums: Vec<char> = number.chars().collect();
    for i in 1..nums.len() {
        if nums[i] == nums [i-1] {
            return false;
        }
    }
    return true;
}

fn no_pair(number: &String) -> bool {
    let nums: Vec<char> = number.chars().collect();
    let mut counts: HashMap<&char, i32> = HashMap::new();
    for n in &nums {
        counts.insert(n, 0);
    }
    for n in &nums {
        let num = counts.get(&n).unwrap() + 1;
        counts.insert(n, num);
    }
    for value in counts.values() {
        if *value == 2 {
            return false
        }
    }
    return true
}