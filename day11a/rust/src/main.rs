use std::fs;

const INPUT_LOCATION: &str = "./input.txt";
const SEPARATOR: &str = " ";
const BLINKS: usize = 25;

fn tranform_stones(stones: &Vec<usize>) -> Vec<usize> {
    let stones = stones.clone();
    let mut new_stones: Vec<usize> = vec![];

    for stone in stones.iter() {
        if *stone == 0 {
            new_stones.push(1);

            continue;
        }

        let stone_string = stone.to_string();
        if stone_string.len() % 2 == 0 {
            let stone_string_split = stone_string.len() / 2;
            new_stones.push(
                stone_string[0..stone_string_split]
                    .parse::<usize>()
                    .unwrap(),
            );
            new_stones.push(
                stone_string[stone_string_split..stone_string.len()]
                    .parse::<usize>()
                    .unwrap(),
            );

            continue;
        }

        new_stones.push(stone * 2024);
    }

    new_stones
}

fn main() {
    let input = fs::read_to_string(INPUT_LOCATION).expect("No input file found");
    let mut stones = input
        .trim()
        .split(SEPARATOR)
        .map(|item| item.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    for _ in 0..BLINKS {
        stones = tranform_stones(&stones);
    }

    println!("Stones after {} blinks: {}", BLINKS, stones.len());
}
