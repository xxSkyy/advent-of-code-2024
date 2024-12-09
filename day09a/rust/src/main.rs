use std::fs;

const INPUT_LOCATION: &str = "./input.txt";
const EMPTY_SYMBOL: isize = -1;

fn main() {
    let input = fs::read_to_string(INPUT_LOCATION).expect("No input file found");

    let mut checksum = 0;
    let mut id: isize = 0;
    let mut is_file = true;
    let mut blocks: Vec<isize> = vec![];
    let mut file_blocks_count: isize = 0;

    input.trim().chars().for_each(|c| {
        let num = c.to_digit(10).unwrap();

        let symbol = if is_file { id } else { EMPTY_SYMBOL };

        for _ in 0..num {
            blocks.push(symbol);
            if is_file {
                file_blocks_count += 1;
            }
        }

        if is_file {
            id += 1;
        }

        is_file = !is_file;
    });

    let mut empty_idxs = blocks
        .clone()
        .into_iter()
        .enumerate()
        .filter(|(_, block)| *block == EMPTY_SYMBOL)
        .map(|(index, _)| index);

    for block_idx in (file_blocks_count..blocks.len() as isize).rev() {
        if blocks[block_idx as usize] == EMPTY_SYMBOL {
            continue;
        };
        let empty_idx = empty_idxs.next().unwrap();

        blocks.swap(empty_idx, block_idx as usize);
    }

    let _ = blocks.split_off(file_blocks_count as usize);

    blocks.into_iter().enumerate().for_each(|(index, number)| {
        if number == EMPTY_SYMBOL {
            return;
        }
        checksum += index * usize::try_from(number).unwrap();
    });

    println!("Checksum: {}", checksum);
}
