use std::{collections::HashMap, fs};

const SEPARATOR: &str = "   ";
const INPUT_LOCATION: &str = "./input.txt";

fn main() {
    let mut simmilarity = 0;

    let mut list_0: Vec<i32> = vec![];
    let mut list_1_appearances: HashMap<i32, i32> = HashMap::new();

    fs::read_to_string(INPUT_LOCATION)
        .expect("No input file found")
        .lines()
        .for_each(|line| {
            let values: Vec<&str> = line.split(SEPARATOR).collect();

            list_0.push(values[0].parse().expect("Failed to parse number"));

            let list_1_val = list_1_appearances
                .entry(values[1].parse().expect("Failed to parse number"))
                .or_insert(0);
            *list_1_val += 1;
        });

    for point in list_0.iter() {
        let appear_count = list_1_appearances.get(point).unwrap_or(&0);

        simmilarity += point * appear_count;
    }

    println!("Simmilarity: {}", simmilarity);
}
