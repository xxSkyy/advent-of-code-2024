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

            let rules_aplies: Vec<Vec<usize>> = rules
                .iter()
                .filter(|rule| numbers.contains(&rule[0]) && numbers.contains(&rule[1]))
                .map(|rule| rule.clone())
                .collect();

            let is_correct = rules_aplies.iter().all(|rules| {
                let first = numbers.iter().position(|n| *n == rules[0]);
                let second = numbers.iter().position(|n| *n == rules[1]);

                if let (Some(first_index), Some(second_index)) = (first, second) {
                    return first_index < second_index;
                }

                false
            });

            if is_correct {
                return acc + numbers[numbers.len() / 2];
            }

            acc
        });

    println!("Sum: {}", sum);
}
