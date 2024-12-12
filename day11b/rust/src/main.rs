use rayon::prelude::*;
use std::{
    fs,
    sync::{Arc, Mutex},
};

const INPUT_LOCATION: &str = "./input.txt";
const SEPARATOR: &str = " ";
const BLINKS: usize = 75;

fn tranform_stones(stones: &mut Vec<usize>) {
    let additions: Arc<Mutex<Vec<usize>>> = Arc::new(Mutex::new(vec![]));

    stones.par_iter_mut().for_each(|stone| {
        if *stone == 0 {
            *stone = 1;

            return;
        }

        let stone_string = stone.to_string();
        if stone_string.len() % 2 == 0 {
            let stone_string_split = stone_string.len() / 2;
            *stone = stone_string[0..stone_string_split]
                .parse::<usize>()
                .unwrap();

            additions.lock().unwrap().push(
                stone_string[stone_string_split..stone_string.len()]
                    .parse::<usize>()
                    .unwrap(),
            );

            return;
        }

        *stone *= 2024;
    });

    stones.append(&mut additions.lock().unwrap());
}

fn main() {
    let input = fs::read_to_string(INPUT_LOCATION).expect("No input file found");
    let mut stones = input
        .trim()
        .split(SEPARATOR)
        .map(|item| item.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    for n in 0..BLINKS {
        tranform_stones(&mut stones);
        println!("{}/{} STONES: ", n, BLINKS);
    }

    println!("Stones after {} blinks: {:?}", BLINKS, stones.len());
}
