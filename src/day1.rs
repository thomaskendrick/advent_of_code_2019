use math::round;

#[aoc(day1, part1)]
fn solve_part_1(input: &str) -> i32{
    let mut total_fuel: i32 = 0;
    for mass in input.lines() {
        let fuel = calculate_base_fuel_requirement(mass.parse().unwrap());
        total_fuel += fuel;
    }
    return total_fuel;
}

fn calculate_base_fuel_requirement(mass: i32) -> i32 {
    let required_fuel = round::floor(mass as f64 / 3.0, 0) as i32 - 2;
    return required_fuel;
}

#[cfg(test)]
mod part_1_tests{
    use super::*;
    #[test]
    fn test_example_1(){
        let result = calculate_base_fuel_requirement(12);
        assert_eq!(result, 2);
    }
    #[test]
    fn test_example_2(){
        let result = calculate_base_fuel_requirement(14);
        assert_eq!(result, 2);
    }
    #[test]
    fn test_example_3(){
        let result = calculate_base_fuel_requirement(1969);
        assert_eq!(result, 654);
    }
    #[test]
    fn test_example_4(){
        let result = calculate_base_fuel_requirement(100756);
        assert_eq!(result, 33583);
    }
}
