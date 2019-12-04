#[aoc(day4, part1)]
fn solve_part_1(string: &str) -> usize {
    let range_bounds: Vec<usize> = string.split("-").map(|r| r.parse().unwrap()).collect();

    let mut number_acceptable_values = 0;

    for value in range_bounds[0]..range_bounds[1] {
        let mut prev_value: usize = 0;
        let mut has_pair = false;
        let is_increasing = num_digits(value).iter().fold(true, |acc, x| {
            let mut does_increase = false;
            if *x == prev_value {
                has_pair = true;
                does_increase = true;
            } else if x > &prev_value {
                does_increase = true;
            }
            prev_value = *x;
            return does_increase && acc;
        });
        if is_increasing && has_pair {
            number_acceptable_values += 1;
        }
    }
    return number_acceptable_values;
}
#[aoc(day4, part2)]
fn solve_part_2(string: &str) -> usize {
    let range_bounds: Vec<usize> = string.split("-").map(|r| r.parse().unwrap()).collect();

    let mut number_acceptable_values = 0;

    for value in range_bounds[0]..range_bounds[1] {
        let mut prev_value: usize = 0;
        let mut counts: [usize; 10]= [0,0,0,0,0,0,0,0,0,0];
        let is_increasing = num_digits(value).iter().fold(true, |acc, x| {
            let mut does_increase = false;
            if *x == prev_value && counts[*x] == 2  {
                does_increase = true;
                counts[*x] += 1;
            } else if *x == prev_value && counts[*x] == 0{
                does_increase = true;
                counts[*x] = 2
            } else if x >= &prev_value {
                does_increase = true;
            }
            prev_value = *x;
            return does_increase && acc;
        });
        if is_increasing && counts.contains(&2) {
            number_acceptable_values += 1;
        }
    }
    return number_acceptable_values;
}
pub fn num_digits(num: usize) -> Vec<usize> {
    num.to_string()
        .chars()
        .filter_map(|x| x.to_digit(10))
        .map(|x| (x as usize))
        .collect()
}
// #[aoc(day4, part2)]
