use std::fs;

const SEPARATOR: &str = "   ";
const INPUT_LOCATION: &str = "./input.txt";

fn main() {
    let mut distance = 0;

    let mut list_0: Vec<i32> = vec![];
    let mut list_1: Vec<i32> = vec![];

    fs::read_to_string(INPUT_LOCATION)
        .expect("No input file found")
        .lines()
        .for_each(|line| {
            let values: Vec<&str> = line.split(SEPARATOR).collect();

            list_0.push(values[0].parse().expect("Failed to parse number"));
            list_1.push(values[1].parse().expect("Failed to parse number"));
        });

    list_0.sort();
    list_1.sort();

    for (point_0, point_1) in list_0.iter().zip(list_1) {
        distance += (point_0 - point_1).abs();
    }

    println!("Distance: {}", distance);
}
