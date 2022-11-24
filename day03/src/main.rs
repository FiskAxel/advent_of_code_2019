fn main() {
    let input = std::fs::read_to_string("src/input.txt")
        .expect("Unreadable");
    let directions: Vec<&str> = input.split('\n').collect();
    let wire1_directions: Vec<&str> = directions[0].split(',').collect();
    let wire2_directions: Vec<&str> = directions[1].split(',').collect();

    let wire1: Vec<Vec<i32>> = get_turning_points(wire1_directions.clone());
    let wire2: Vec<Vec<i32>> = get_turning_points(wire2_directions.clone());

    let mut intersections: Vec<Vec<i32>> = Vec::new();
    for i in 0..wire1.len() - 1 {
        for j in 0..wire2.len() - 1 {
            let intersection = get_intersection(
                wire1[i].clone(), wire1[i + 1].clone(), 
                wire2[j].clone(), wire2[j + 1].clone());
            if intersection[0] == 0 && intersection[1] == 0 {
                continue; // No intersection
            }
            intersections.push(intersection);
        }
    }

    let mut closest_part1: i32 = i32::MAX;
    let mut closest_part2: i32 = i32::MAX;
    for intersection in intersections {
        let ix = intersection[0];
        let iy = intersection[1];

        let manhattan_distance = ix.abs() + iy.abs();
        if manhattan_distance < closest_part1 {
            closest_part1 = manhattan_distance;
        }

        let w1 = get_wires_distance_to_intersection_point(wire1_directions.clone(), ix, iy);
        let w2 = get_wires_distance_to_intersection_point(wire2_directions.clone(), ix, iy);
        if (w1 + w2) < closest_part2 {
            closest_part2 = w1 + w2;
        }
    }
    println!("Part 1: {}", closest_part1);
    println!("Part 2: {}", closest_part2);


}

fn get_turning_points(wire_directions: Vec<&str>) -> Vec<Vec<i32>> {
    let mut wire: Vec<Vec<i32>> = vec![vec![0,0]];
    
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    for dir in wire_directions {
        let udlr: &str = &dir[0..1];
        let distance: &i32 = &dir[1..].trim().parse::<i32>().expect("parse error");
        if udlr == "U" {
            y += distance;
        } else if udlr == "D" {
            y -= distance;
        } else if udlr == "L" {
            x -= distance;
        } else if udlr == "R" {
            x += distance;
        }
        wire.push(vec![x, y]);
    }
    return wire;
}

fn get_intersection(line_a1: Vec<i32>, line_a2: Vec<i32>, line_b1: Vec<i32>, line_b2: Vec<i32>) -> Vec<i32> {
    let mut intersection: Vec<i32> = vec![0, 0];
    
    let a_x1 = line_a1[0];
    let a_y1 = line_a1[1];
    let a_x2 = line_a2[0];
    let a_y2 = line_a2[1];
    
    let b_x1 = line_b1[0];
    let b_y1 = line_b1[1];
    let b_x2 = line_b2[0];
    let b_y2 = line_b2[1];

    if (a_x1 == a_x2) && (b_y1 == b_y2) {
        if ((b_x1 < a_x1 && a_x1 < b_x2) ||
            (b_x2 < a_x1 && a_x1 < b_x1)) && 
           ((a_y1 < b_y1 && b_y1 < a_y2) ||
            (a_y2 < b_y1 && b_y1 < a_y1)) {
            intersection = vec![a_x1, b_y1];
        }
    } else if (b_x1 == b_x2) && (a_y1 == a_y2) {
        if ((a_x1 < b_x1 && b_x1 < a_x2) ||
            (a_x2 < b_x1 && b_x1 < a_x1)) && 
           ((b_y1 < a_y1 && a_y1 < b_y2) ||
            (b_y2 < a_y1 && a_y1 < b_y1)) {
            intersection = vec![b_x1, a_y1];
        }
    }

    return intersection;
}

// Slow
fn get_wires_distance_to_intersection_point(wire_directions: Vec<&str>, ix: i32, iy: i32) -> i32 {
    let mut tot_distance = 0;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    for dir in wire_directions {
        let udlr: &str = &dir[0..1];
        let distance: &i32 = &dir[1..].trim().parse::<i32>().expect("parse error");
        for _ in 0..*distance {
            if udlr == "U" {
                y += 1;
            } else if udlr == "D" {
                y -= 1;
            } else if udlr == "L" {
                x -= 1;
            } else if udlr == "R" {
                x += 1;
            }
            tot_distance += 1; 
            
            if x == ix && y == iy { // At intersection
                return tot_distance;
            }
        };
    }
    return 0;
    
}