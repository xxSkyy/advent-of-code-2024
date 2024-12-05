use std::fs;

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
            let numbers: Vec<usize> = item
                .split(",")
                .map(|n| n.parse::<usize>().unwrap())
                .collect();

            let applied_rules: Vec<Vec<usize>> = rules
                .iter()
                .filter(|rule| numbers.contains(&rule[0]) && numbers.contains(&rule[1]))
                .map(|rule| rule.clone())
                .collect();

            let is_correct = applied_rules.iter().all(|rules| {
                let first_index = numbers.iter().position(|n| *n == rules[0]).unwrap();
                let second_index = numbers.iter().position(|n| *n == rules[1]).unwrap();

                first_index < second_index
            });

            if is_correct {
                return acc + numbers[numbers.len() / 2];
            }

            acc
        });

    println!("Sum: {}", sum);
}
