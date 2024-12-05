use std::{cmp::Ordering, fs};

const INPUT_LOCATION: &str = "./input.txt";

fn main() {
    let input = fs::read_to_string(INPUT_LOCATION).expect("No input file found");
    let input_vec: Vec<&str> = input.trim().split("\n").collect();

    let split_position = input_vec.iter().position(|item| item == &"").unwrap();
    let rules: &Vec<Vec<usize>> = &input_vec[0..split_position]
        .iter()
        .map(|item| {
            item.split("|")
                .map(|n| n.parse::<usize>().unwrap())
                .collect()
        })
        .collect();

    let sum = &input_vec[split_position + 1..input_vec.len()]
        .iter()
        .fold(0, |acc, item| {
            let mut numbers: Vec<usize> = item
                .split(",")
                .map(|n| n.parse::<usize>().unwrap())
                .collect();

            let rules_aplies: Vec<Vec<usize>> = rules
                .iter()
                .filter(|rule| numbers.contains(&rule[0]) && numbers.contains(&rule[1]))
                .map(|rule| rule.clone())
                .collect();

            let is_correct = rules_aplies.iter().all(|rules| {
                let first_index = numbers.iter().position(|n| *n == rules[0]).unwrap();
                let second_index = numbers.iter().position(|n| *n == rules[1]).unwrap();

                first_index < second_index
            });

            if is_correct {
                return acc;
            }

            numbers.sort_by(|a, b| {
                let rule = rules_aplies
                    .iter()
                    .find(|rule| rule.contains(a) && rule.contains(b));

                if let None = rule {
                    return Ordering::Equal;
                }

                if &rule.unwrap()[0] == a {
                    return Ordering::Less;
                }
                return Ordering::Greater;
            });

            return acc + numbers[numbers.len() / 2];
        });

    println!("Sum: {}", sum);
}
