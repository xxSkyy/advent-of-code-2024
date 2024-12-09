use std::{collections::HashMap, fs};

const INPUT_LOCATION: &str = "./input.txt";
const EMPTY_SYMBOL: isize = -1;

struct Block {
    id: usize,
    index: isize,
    length: isize,
}

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

    let mut empty_idxs: Vec<_> = blocks
        .clone()
        .into_iter()
        .enumerate()
        .filter(|(_, block)| *block == EMPTY_SYMBOL)
        .map(|(index, _)| index)
        .collect::<Vec<_>>();

    let mut blocks_map: HashMap<isize, Block> = HashMap::new();

    for (index, block) in blocks.iter().enumerate() {
        if *block == EMPTY_SYMBOL {
            continue;
        };
        blocks_map
            .entry(*block)
            .and_modify(|entry| entry.length += 1)
            .or_insert(Block {
                id: *block as usize,
                index: index as isize,
                length: 1,
            });
    }

    let mut blocks_vec = Vec::from_iter(blocks_map.values());
    blocks_vec.sort_by_key(|block| block.id);

    for block in blocks_vec.into_iter().rev() {
        let start_idx = block.index;
        let file_size = block.length;

        let mut b_index: isize = -1;
        let mut e_index: isize = -1;

        let mut found = false;
        for (index, blocks_index) in empty_idxs.iter().enumerate().collect::<Vec<_>>() {
            if *blocks_index >= start_idx as usize {
                break;
            }

            for n in 0..file_size {
                let get_item = empty_idxs.get(index + n as usize);
                if let Some(item) = get_item {
                    if *item == *blocks_index + n as usize && n == file_size - 1 {
                        found = true;
                        b_index = (*blocks_index).clone() as isize;
                        e_index = index.clone() as isize;
                        break;
                    }
                }
            }
        }

        if found {
            for n in 0..file_size {
                blocks.swap(
                    start_idx as usize + n as usize,
                    b_index as usize + n as usize,
                );
                empty_idxs.remove(e_index as usize);
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
