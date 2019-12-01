use math::round;

#[aoc(day1, part1)]
fn solve_part_1(input: &str) -> i32{
    let mut total_fuel: i32 = 0;
    for mass in input.lines() {
        let fuel = calculate_fuel_requirement(mass.parse().unwrap());
        total_fuel += fuel;
    }
    return total_fuel;
}

#[aoc(day1, part2)]
fn solve_part_2(input: &str) -> i32{
    let mut total_fuel: i32 = 0;
    for mass in input.lines() {
        let fuel = calculate_fuel_requirement(mass.parse().unwrap());
        let additional_fuel = calculate_additional_fuel(fuel);
        total_fuel += fuel;
        total_fuel += additional_fuel;
    }
    return total_fuel;
}

fn calculate_fuel_requirement(mass: i32) -> i32 {
    let required_fuel = round::floor(mass as f64 / 3.0, 0) as i32 - 2;
    return required_fuel;
}

fn calculate_additional_fuel(base_fuel: i32) -> i32 {
    let mut total_additional_fuel: i32 = 0;
    let mut additional_fuel = base_fuel;
    loop {
        additional_fuel = calculate_fuel_requirement(additional_fuel);
        if additional_fuel > 0 {
            total_additional_fuel += additional_fuel;
        } else {
            break;
        }
    }
    return total_additional_fuel;
}

#[cfg(test)]
mod part1{
    use super::*;
    #[test]
    fn test_example_1(){
        let result = calculate_fuel_requirement(12);
        assert_eq!(result, 2);
    }
    #[test]
    fn test_example_2(){
        let result = calculate_fuel_requirement(14);
        assert_eq!(result, 2);
    }
    #[test]
    fn test_example_3(){
        let result = calculate_fuel_requirement(1969);
        assert_eq!(result, 654);
    }
    #[test]
    fn test_example_4(){
        let result = calculate_fuel_requirement(100756);
        assert_eq!(result, 33583);
    }
}

#[cfg(test)]
mod part2{
    use super::*;
    #[test]
    fn test_example_1(){
        let base_fuel = 2;
        let result = base_fuel + calculate_additional_fuel(base_fuel);
        assert_eq!(result, 2);
    }
    #[test]
    fn test_example_2(){
        let base_fuel = 654;
        let result = base_fuel + calculate_additional_fuel(base_fuel);
        assert_eq!(result, 966);
    }
    #[test]
    fn test_example_3(){
        let base_fuel = 33583;
        let result = base_fuel + calculate_additional_fuel(base_fuel);
        assert_eq!(result, 50346);
    }
}
