#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> (Vec<(i32, i32)>, Vec<(i32, i32)>) {
    let mut lines = input.lines();
    let line_1 = lines.next().unwrap();
    let line_2 = lines.next().unwrap();
    let path_1 = process_line(line_1);
    let path_2 = process_line(line_2);
    return (path_1, path_2);
}

fn process_line(line: &str) -> Vec<(i32, i32)> {
    let mut current_x: i32 = 0;
    let mut current_y: i32 = 0;
    let mut path: Vec<(i32, i32)> = Vec::new();
    for instruction in line.split(",") {
        let direction = instruction.get(..1);
        match direction {
            Some("U") => {
                let distance: i32 = instruction.get(1..).unwrap().parse().unwrap();
                for _ in 0..distance {
                    current_y += 1;
                    path.push((current_x, current_y));
                }
            }
            Some("D") => {
                let distance: i32 = instruction.get(1..).unwrap().parse().unwrap();
                for _ in 0..distance {
                    current_y -= 1;
                    path.push((current_x, current_y));
                }
            }
            Some("L") => {
                let distance: i32 = instruction.get(1..).unwrap().parse().unwrap();
                for _ in 0..distance {
                    current_x -= 1;
                    path.push((current_x, current_y));
                }
            }
            Some("R") => {
                let distance: i32 = instruction.get(1..).unwrap().parse().unwrap();
                for _ in 0..distance {
                    current_x += 1;
                    path.push((current_x, current_y));
                }
            }
            Some(_) => panic!("Couldn't find direction!"),
            None => panic!("Couldn't find direction!"),
        }
    }
    return path;
}

#[aoc(day3, part1)]
fn solve_part_1(input: &(Vec<(i32, i32)>, Vec<(i32, i32)>)) -> i32 {
    let (path_1, path_2) = input.clone();
    let mut manhattans: Vec<i32> = Vec::new();

    for coords1 in &path_1 {
        for coords2 in &path_2 {
            if coords1.0 == coords2.0 && coords1.1 == coords2.1 {
                manhattans.push(coords1.0.abs() + coords1.1.abs())
            }
        }
    }
    manhattans.sort();
    return manhattans[0];
}

#[aoc(day3, part2)]
fn solve_part_2(input: &(Vec<(i32, i32)>, Vec<(i32, i32)>)) -> i32 {
    let (path_1, path_2) = input.clone();
    let mut combined_steps: Vec<i32> = Vec::new();
    let mut steps1 = 0;
    let mut steps2 = 0;

    for coords1 in &path_1 {
        steps1 += 1;
        for coords2 in &path_2 {
            steps2 += 1;
            if coords1.0 == coords2.0 && coords1.1 == coords2.1 {
                combined_steps.push(steps1 + steps2)
            }
        }
        steps2 = 0;
    }
    combined_steps.sort();

    return combined_steps[0];
}
