use std::fs;

use itertools::Itertools;
use regex::Regex;

const INPUT_LOCATION: &str = "./input.txt";

fn main() {
    let input = fs::read_to_string(INPUT_LOCATION).expect("No input file found");

    let mut correct_sum: usize = 0;
    let symbols = ["+", "*"];

    let line_regex = Regex::new(r"(.*\d): (.*\d)").expect("Regex issue");

    let data: Vec<Vec<usize>> = input
        .trim()
        .lines()
        .map(|line| {
            let captures = line_regex.captures(line).unwrap();
            let mut result = vec![captures[1].parse::<usize>().unwrap()];
            let mut numbers: Vec<usize> = captures[2]
                .to_string()
                .split(" ")
                .map(|n| n.parse::<usize>().unwrap())
                .collect();

            result.append(&mut numbers);

            result
        })
        .collect();

    data.iter().for_each(|entry| {
        let mut numbers = entry.clone();
        let desired_sum = numbers.remove(0);

        let combinations_input = vec![symbols.as_slice(); numbers.len() - 1];
        let combinations = combinations_input.into_iter().multi_cartesian_product();

        for combination in combinations {
            let sum: usize =
                numbers
                    .clone()
                    .into_iter()
                    .enumerate()
                    .fold(0, |acc, (index, number)| {
                        if acc == 0 {
                            return number;
                        };

                        match combination[index - 1] {
                            &"+" => acc + number,
                            _ => acc * number,
                        }
                    });

            if desired_sum == sum {
                correct_sum += sum;
                break;
            }
        }
    });

    println!("Correct sum: {}", correct_sum);
}
