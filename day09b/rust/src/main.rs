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

    for block_id in (0..id).rev() {
        let items: Vec<_> = blocks
            .iter()
            .enumerate()
            .filter(|(_, block)| **block == block_id)
            .collect();

        let start_idx = items[0].0;
        let file_size = items.len();

        drop(items);

        let space = blocks.iter().enumerate().position(|(index, _)| {
            let mut found = true;
            if index >= start_idx {
                return false;
            }

            for n in 0..file_size {
                let n_item = blocks.get(index + n);
                if let Some(block) = n_item {
                    if *block == EMPTY_SYMBOL {
                        continue;
                    }
                }

                found = false;
                break;
            }

            found
        });

        if let Some(free_index) = space {
            for n in 0..file_size {
                blocks.swap(start_idx + n, free_index + n);
            }
        }
    }

    blocks.into_iter().enumerate().for_each(|(index, number)| {
        if number == EMPTY_SYMBOL {
            return;
        }
        checksum += index * usize::try_from(number).unwrap();
    });

    println!("Checksum: {}", checksum);
}
