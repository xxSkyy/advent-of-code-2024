use regex::Regex;
use std::fs;

const INPUT_LOCATION: &str = "./input.txt";

const ENABLE_INSTRUCTION: &str = "do()";
const DISABLE_INSTRUCTION: &str = "don't()";
const MULTIPLY_INSTRUCTION: &str = "mul(";

fn main() {
    let mut sum = 0;
    let mut is_enabled = true;

    let input = fs::read_to_string(INPUT_LOCATION).expect("No input file found");

    let mul_regex = Regex::new(r"mul\((\d*),(\d*)\)|do\(\)|don\'t\(\)").expect("Regex issue");

    for capture in mul_regex.captures_iter(&input) {
        if capture[0].eq(ENABLE_INSTRUCTION) {
            is_enabled = true;
        }

        if capture[0].eq(DISABLE_INSTRUCTION) {
            is_enabled = false;
        }

        if capture[0].starts_with(MULTIPLY_INSTRUCTION) && is_enabled {
            let num1 = &capture[1].parse::<i32>().expect("Number parse error");
            let num2 = &capture[2].parse::<i32>().expect("Number parse error");

            sum += num1 * num2;
        }
    }

    println!("Sum of multiplications: {}", sum);
}
