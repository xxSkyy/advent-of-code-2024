use regex::Regex;
use std::fs;

const INPUT_LOCATION: &str = "./input.txt";

fn main() {
    let mut sum = 0;

    let input = fs::read_to_string(INPUT_LOCATION).expect("No input file found");

    let mul_regex = Regex::new(r"mul\((\d*),(\d*)\)").expect("Regex issue");

    for capture in mul_regex.captures_iter(&input) {
        let num1 = &capture[1].parse::<i32>().expect("Number parse error");
        let num2 = &capture[2].parse::<i32>().expect("Number parse error");

        sum += num1 * num2;
    }

    println!("Sum of multiplications: {}", sum);
}
