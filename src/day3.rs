struct Point<T>{
    x: T,
    y: T 
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> () {
    let current_x: i32;
    let current_y: i32;
    let line_1 = input.lines().next().unwrap();
    let line_2 = input.lines().next().unwrap();
    let path_1 = process_line(line_1);
    let path_2 = process_line(line_2);
}

fn process_line (line: &str) -> Vec<Point<i32>>{
    let mut current_x: i32 = 0;
    let mut current_y: i32 = 0;
    let mut path: Vec<Point<i32>> = Vec::new();
    for instruction in line.split(","){
        let direction = instruction.get(..1);
        match direction {
            Some("U") => {
                let distance: i32 = instruction.get(1..).unwrap().parse().unwrap();
                for _ in 0..distance {
                    current_y += 1;
                    path.push(Point{x : current_x, y: current_y});
        println!("Pushed x={} y={}", current_x, current_y);
                }
            },
            Some("D") => {
                let distance: i32 = instruction.get(1..).unwrap().parse().unwrap();
                for _ in 0..distance {
                    current_y -= 1;
                    path.push(Point{x : current_x, y: current_y});
        println!("Pushed x={} y={}", current_x, current_y);
                }
            },
            Some("L") => {
                let distance: i32 = instruction.get(1..).unwrap().parse().unwrap();
                for _ in 0..distance {
                    current_x -= 1;
                    path.push(Point{x : current_x, y: current_y});
        println!("Pushed x={} y={}", current_x, current_y);
                }
            },
            Some("R") => {
                let distance: i32 = instruction.get(1..).unwrap().parse().unwrap();
                for _ in 0..distance {
                    current_x += 1;
                    path.push(Point{x : current_x, y: current_y});
        println!("Pushed x={} y={}", current_x, current_y);
                }
            },
            Some(_) =>panic!("Couldn't find direction!"),
            None => panic!("Couldn't find direction!")
        }
    }
    return path;
}

#[aoc(day3, part1)]
fn solve_part_1(input: &()) -> i32 {
    return 0;
}

// #[aoc(day2, part2)]
// fn solve_part_2(input: &Vec<usize>) -> usize {
// }

#[cfg(test)]
mod part1 {
    use super::*;
    #[test]
    fn test_example_1() {
        let result = process_code(vec![1, 0, 0, 0, 99]);
        assert_eq!(result, [2, 0, 0, 0, 99]);
    }
    #[test]
    fn test_example_2() {
        let result = process_code(vec![2, 3, 0, 3, 99]);
        assert_eq!(result, vec![2, 3, 0, 6, 99]);
    }
    #[test]
    fn test_example_3() {
        let result = process_code(vec![2, 4, 4, 5, 99, 0]);
        assert_eq!(result, vec![2, 4, 4, 5, 99, 9801]);
    }
    #[test]
    fn test_example_4() {
        let result = process_code(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]);
        assert_eq!(result, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }
}
