use std::fs;

const INPUT_LOCATION: &str = "./input.txt";

#[derive(Debug, Clone, Copy)]
struct Block {
    id: isize,
    index: isize,
    length: isize,
}

fn main() {
    let input = fs::read_to_string(INPUT_LOCATION).expect("No input file found");

    let mut new_checksum: isize = 0;
    let mut id: isize = 0;
    let mut index: isize = 0;
    let mut is_file = true;

    let mut blocks: Vec<Block> = vec![];
    let mut empty_blocks: Vec<Block> = vec![];

    input.trim().chars().for_each(|c| {
        let num = c.to_digit(10).unwrap();

        if is_file {
            blocks.push(Block {
                id: id as isize,
                index,
                length: num as isize,
            });

            id += 1;
        } else {
            empty_blocks.push(Block {
                id: -1,
                index,
                length: num as isize,
            });
        }

        index += num as isize;
        is_file = !is_file;
    });

    blocks.iter_mut().rev().for_each(|block| {
        let empty_block = empty_blocks.iter_mut().find(|empty_block| {
            block.length <= empty_block.length && block.index > empty_block.index
        });
        if let None = empty_block {
            return;
        }
        let empty_block = empty_block.unwrap();

        block.index = empty_block.index;

        empty_block.length -= block.length;
        empty_block.index += block.length;
    });

    blocks.sort_by_key(|block| block.index);

    blocks.iter().for_each(|block| {
        if block.id == -1 {
            return;
        }
        for n in 0..block.length {
            new_checksum += block.id * (block.index + n) as isize;
        }
    });

    println!("CHECKSUM: {}", new_checksum);
}
