fn main() {
    let input = std::fs::read_to_string("src/input.txt")
        .expect("Unreadable");
    let intcode: Vec<i32> = input
        .split(',')
        .map(|n| n.parse::<i32>().expect("parse error"))
        .collect();

    let part1: i32 = intcode_program(intcode.clone(), 12, 2);
    println!("Part 1: {}", part1);

    let mut part2: i32;
    let mut noun = 1;
    let mut verb = 1;
    loop {
        part2 = intcode_program(intcode.clone(), noun, verb);
        if part2 == 19690720 {
            
            break
        }
        noun += 1;
        if noun == 100 {
            noun = 1;
            verb += 1;
        }
    }
    println!("Part 2: {}", 100 * noun + verb);
}

fn intcode_program(mut intcode: Vec<i32>, noun: i32, verb: i32) -> i32 {
    intcode[1] = noun;
    intcode[2] = verb;

    let mut i: usize = 0;
    loop {
        let opcode: i32 = intcode[i];
        if opcode == 99 {
            return intcode[0]
        }

        let pos1: i32 = intcode[i + 1];
        let pos2: i32 = intcode[i + 2];
        let target: i32 = intcode[i + 3];
        
        if opcode == 1 {
            intcode[target as usize] = intcode[pos1 as usize] + intcode[pos2 as usize];
        } else if opcode == 2 {
            intcode[target as usize] = intcode[pos1 as usize] * intcode[pos2 as usize];
        }
        i += 4;
    }
}
