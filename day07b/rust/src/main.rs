use itertools::Itertools;
use rayon::prelude::*;
use regex::Regex;
use std::{
    fs,
    sync::{Arc, Mutex},
};

const INPUT_LOCATION: &str = "./input.txt";

fn main() {
    let input = fs::read_to_string(INPUT_LOCATION).expect("No input file found");

    let correct_sum = Arc::new(Mutex::new(0));
    let symbols = ["+", "*", "||"];

    let line_regex = Regex::new(r"(.*\d): (.*\d)").expect("Regex issue");

    let data: Vec<Vec<isize>> = input
        .trim()
        .lines()
        .map(|line| {
            let captures = line_regex.captures(line).unwrap();
            let mut result = vec![captures[1].parse::<isize>().unwrap()];
            let mut numbers: Vec<isize> = captures[2]
                .to_string()
                .split(" ")
                .map(|n| n.parse::<isize>().unwrap())
                .collect();

            result.append(&mut numbers);

            result
        })
        .collect();

    data.par_iter().for_each(|entry| {
        let mut numbers = entry.clone();
        let desired_sum = numbers.remove(0);

        let combinations_input = vec![symbols.as_slice(); numbers.len() - 1];
        let combinations = combinations_input.into_iter().multi_cartesian_product();

        for combination in combinations {
            let sum: isize =
                numbers
                    .clone()
                    .into_iter()
                    .enumerate()
                    .fold(0, |acc, (index, number)| {
                        let number = isize::try_from(number).unwrap();

                        if acc == 0 {
                            return number;
                        };

                        match combination[usize::try_from(index - 1).unwrap()] {
                            &"+" => acc + number,
                            &"*" => acc * number,
                            _ => format!("{}{}", acc, number).parse::<isize>().unwrap(),
                        }
                    });

            if desired_sum == sum {
                *correct_sum.lock().unwrap() += sum;
                break;
            }
        }
    });

    println!("Correct sum: {}", correct_sum.lock().unwrap());
}
