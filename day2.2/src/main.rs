use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Can't read file");

    let items: Vec<usize> = input
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    for noun in 0..99 {
        for verb in 0..99 {
            let mut items_modified = items.to_vec();
            items_modified[1] = noun;
            items_modified[2] = verb;

            let mut i = 0;

            while i < items.len() {
                let output_index = items_modified[i + 3];
                let operand1_index = items_modified[i + 1];
                let operand2_index = items_modified[i + 2];

                match FromPrimitive::from_usize(items_modified[i]) {
                    Some(OpCodes::Add) => {
                        items_modified[output_index] =
                            items_modified[operand1_index] + items_modified[operand2_index];
                        i += 4;
                    }
                    Some(OpCodes::Multiply) => {
                        items_modified[output_index] =
                            items_modified[operand1_index] * items_modified[operand2_index];
                        i += 4;
                    }
                    Some(OpCodes::Halt) => {
                        break;
                    }
                    None => {
                        i += 1;
                    }
                }
            }

            if items_modified[0] == 19690720 {
                println!("item found noun:{}, verb:{}", noun, verb);
                println!("{}", noun * 100 + verb);
                break;
            }
        }
    }
}

#[derive(FromPrimitive)]
enum OpCodes {
    Add = 1,
    Multiply = 2,
    Halt = 99,
}
