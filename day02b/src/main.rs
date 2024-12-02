use std::fs;

const SEPARATOR: &str = " ";
const INPUT_LOCATION: &str = "./input.txt";

const MIN_LEVEL_DIFF: i32 = 1;
const MAX_LEVEL_DIFF: i32 = 3;

fn get_direction(input: &i32) -> i32 {
    if *input < 0 {
        -1
    } else {
        1
    }
}

fn is_report_correct(level: &Vec<i32>) -> bool {
    let mut is_report_correct = true;

    let mut report_iter = level.iter();

    let mut last_direction = 0;
    let mut last_level = report_iter.next().unwrap();

    report_iter.for_each(|level| {
        let difference = last_level - level;
        let direction = get_direction(&difference);

        let is_too_much_difference =
            difference.abs() < MIN_LEVEL_DIFF || difference.abs() > MAX_LEVEL_DIFF;
        let is_bad_direction = last_direction != 0 && direction != last_direction;

        let is_level_wrong = is_too_much_difference || is_bad_direction;

        if is_level_wrong {
            is_report_correct = false;
            return;
        }

        last_direction = direction;
        last_level = level;
    });

    return is_report_correct;
}

fn main() {
    let mut correct_levels = 0;

    fs::read_to_string(INPUT_LOCATION)
        .expect("No input file found")
        .lines()
        .for_each(|line| {
            let report: Vec<i32> = line
                .split(SEPARATOR)
                .map(|level| -> i32 { level.parse().expect("Failed to parse level to number") })
                .collect();

            let mut variations: Vec<Vec<i32>> = vec![];

            if is_report_correct(&report) {
                correct_levels += 1;
                return;
            }

            for index in 0..report.len() {
                let mut variation = report.clone();

                variation.remove(index);
                variations.push(variation);
            }

            for variation in variations.iter() {
                if is_report_correct(&variation) {
                    correct_levels += 1;
                    break;
                }
            }
        });

    println!("Correct levels: {}", correct_levels);
}
