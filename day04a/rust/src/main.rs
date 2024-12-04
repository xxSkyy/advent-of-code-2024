use std::fs;

use fancy_regex::Regex;

const INPUT_LOCATION: &str = "./input.txt";

fn rotate_vec(input_vec: &Vec<Vec<String>>) -> Vec<Vec<String>> {
    let size = input_vec.len();

    let mut rotated_vec: Vec<Vec<String>> = vec![vec!["".to_string(); size]; size];
    for row in 0..size {
        for col in 0..size {
            rotated_vec[size - col - 1][row] = input_vec[row][col].clone();
        }
    }

    rotated_vec
}

fn get_diagonals(input_vec: &Vec<Vec<String>>) -> Vec<&str> {
    let mut output: Vec<&str> = vec![];
    let size = input_vec.len();

    for diagonal in 0..=(size * 2) {
        for x in 0..=diagonal {
            let y = diagonal - x;
            if y < size && x < size {
                output.push(input_vec.get(y).unwrap().get(x).unwrap());
            }
        }
        output.push("\n")
    }

    output
}

fn main() {
    let input = fs::read_to_string(INPUT_LOCATION).expect("No input file found");

    let input_vec: Vec<Vec<String>> = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_string()).collect::<Vec<String>>())
        .collect();

    let mut result: Vec<&str> = vec![];

    let rotated_vec = rotate_vec(&input_vec);

    result.append(&mut get_diagonals(&input_vec));
    result.append(&mut get_diagonals(&rotated_vec));

    let rotated_input = rotated_vec
        .iter()
        .map(|item| item.iter().map(|item| item.clone()).collect::<String>())
        .map(|item| item + "\n")
        .collect::<String>();

    let result_string =
        input.to_owned() + &rotated_input + &result.iter().map(|item| *item).collect::<String>();

    let xmas_regex = Regex::new(r"(?=XMAS)|(?=SAMX)").expect("Regex issue");
    let result_vec: Vec<&str> = xmas_regex
        .captures_iter(&result_string)
        .map(|_| "")
        .collect();

    println!("{}", result_vec.len());
}
