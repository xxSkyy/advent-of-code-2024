// use rayon::prelude::*;
use std::{collections::HashMap, fs};

const INPUT_LOCATION: &str = "./input.txt";
const SEPARATOR: &str = " ";
const BLINKS: usize = 75;

fn tranform_stones(stones: &mut HashMap<usize, usize>) {
    let stones_keys = stones.clone();

    for (number, amount) in stones_keys.iter() {
        // stones.remove(number);
        *stones.get_mut(number).unwrap() -= amount;

        if *number == 0 {
            let _ = *stones.entry(1).or_insert(0);
            *stones.get_mut(&1).unwrap() += amount;
            continue;
        }

        let stone_string = number.to_string();
        if stone_string.len() % 2 == 0 {
            let stone_string_split = stone_string.len() / 2;

            let num1 = stone_string[0..stone_string_split]
                .parse::<usize>()
                .unwrap();

            let num2 = stone_string[stone_string_split..stone_string.len()]
                .parse::<usize>()
                .unwrap();

            let _ = *stones.entry(num1).or_insert(0);
            *stones.get_mut(&num1).unwrap() += amount;
            let _ = *stones.entry(num2).or_insert(0);
            *stones.get_mut(&num2).unwrap() += amount;

            continue;
        }

        let _ = *stones.entry(number * 2024).or_insert(0);
        *stones.get_mut(&(number * 2024)).unwrap() += amount;
    }
}

fn main() {
    let input = fs::read_to_string(INPUT_LOCATION).expect("No input file found");
    let mut stones_map = HashMap::new();

    input.trim().split(SEPARATOR).for_each(|item| {
        stones_map
            .entry(item.parse::<usize>().unwrap())
            .or_insert(1 as usize);
    });

    for n in 0..BLINKS {
        tranform_stones(&mut stones_map);
        // println!("{}/{} STONES: ", n + 1, BLINKS);
        // println!("{:?}", stones_map);
    }

    let stones_amount = stones_map.values().fold(0, |acc, amount| acc + amount);

    println!("Stones after {} blinks: {:?}", BLINKS, stones_amount);
}
