#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<usize> {
    let code = input.split(",").map(|n| n.parse().unwrap()).collect();
    return code;
}

#[aoc(day2, part1)]
fn solve_part_1(input: &Vec<usize>) -> usize {
    let mut input = input.clone();
    input[1] = 12;
    input[2] = 2;
    let output = process_code(input);
    return output[0];
}

#[aoc(day2, part2)]
fn solve_part_2(input: &Vec<usize>) -> usize {

    let mut result = 0;
    for verb in 0..99 {
        for noun in 0..99 {
            let mut input = input.clone();
            input[1] = noun; 
            input[2] = verb;
            let output = process_code(input);
            if output[0] == 19690720 {
                result = 100 * noun + verb ;
            }
        }
    }
    return result;
}

fn process_code(mut code: Vec<usize>) -> Vec<usize> {
    let mut i = 0;
    loop {
        match code[i] {
            1 => {
                let target = code[i+3];
                code[target] = code[code[i+1]]+ code[code[i+2]]
            },
            2 => {
                let target = code[i+3];
                code[target] = code[code[i+1]] * code[code[i+2]];
            }
            99 => break,
            _ => panic!("Something went wrong")
        }
        i += 4;
    }
    return code;
}

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
