#[aoc(day4, part1)]
fn solve_part_1(string: &str) -> i32 {
    let range_bounds: Vec<i32> = string.split("-").map(|r| r.parse().unwrap()).collect();

    let mut number_acceptable_values = 0;

    for value in range_bounds[0]..range_bounds[1] {
        let mut prev_value: i32 = 0;
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
fn solve_part_2(string: &str) -> i32 {
    let range_bounds: Vec<i32> = string.split("-").map(|r| r.parse().unwrap()).collect();

    let mut number_acceptable_values = 0;

    for value in range_bounds[0]..range_bounds[1] {
        let mut prev_value: i32 = 0;
        let mut has_pair = false;
        let mut existing_pair_groups: Vec<(i32, i32)> = Vec::new();
        let is_increasing = num_digits(value).iter().fold(true, |acc, x| {
            let mut does_increase = false;
            if *x == prev_value && existing_pair_groups.contains(&x,_) {

            } else if *x == prev_value {
                has_pair = true;
                does_increase = true;
                existing_pair_groups.push((*x, 1))
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
pub fn num_digits(num: i32) -> Vec<i32> {
    num.to_string()
        .chars()
        .filter_map(|x| x.to_digit(10))
        .map(|x| (x as i32))
        .collect()
}
// #[aoc(day4, part2)]
