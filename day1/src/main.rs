use std::fs;

fn read_file(file_name: &str) -> String {
    fs::read_to_string(file_name).expect("Input file not found")
}

fn parse(contents: String) -> Vec<i64> {
    contents.split("\n").map(|x| x.parse::<i64>().unwrap()).collect()
}

fn solve_part_1(input: &Vec<i64>) -> Result<i64, &str> {
    for i in 0..input.len() {
        for j in i+1..input.len() {
            if input[i] + input[j] == 2020 {
                return Ok(input[i] * input[j]);
            }
        }
    }
    Err("No solution found")
}

fn solve_part_2(input: &Vec<i64>) -> Result<i64, &str> {
    for i in 0..input.len() {
        for j in i+1..input.len() {
            for k in j+1..input.len() {
                if input[i] + input[j] + input[k] == 2020 {
                    return Ok(input[i] * input[j] * input[k]);
                }
            }
        }
    }
    Err("No solution found")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_no_solution() {
        let test_input = "43\n34";
        let input = parse(test_input.to_string());
        let result = solve_part_1(&input);
        assert_eq!(result, Err("No solution found"));
    }

    #[test]
    fn test_part1_solution_found() {
        let test_input = "1721\n979\n366\n299\n675\n1456";
        let input = parse(test_input.to_string());
        let result = solve_part_1(&input);
        assert_eq!(result, Ok(514579));
    }

    #[test]
    fn test_part2_no_solution() {
        let test_input = "43\n34";
        let input = parse(test_input.to_string());
        let result = solve_part_2(&input);
        assert_eq!(result, Err("No solution found"));
    }

    #[test]
    fn test_part2_solution_found() {
        let test_input = "1721\n979\n366\n299\n675\n1456";
        let input = parse(test_input.to_string());
        let result = solve_part_2(&input);
        assert_eq!(result, Ok(241861950));
    }
}

fn main() {
    let input = parse(read_file("input.txt"));
    let part1_result = solve_part_1(&input);
    match part1_result {
        Ok(result) => println!("Part 1: {}", result),
        Err(err) => println!("Part 1 failed: {}", err),
    };
    let part2_result = solve_part_2(&input);
    match part2_result {
        Ok(result) => println!("Part 2: {}", result),
        Err(err) => println!("Part 2 failed: {}", err),
    };
}