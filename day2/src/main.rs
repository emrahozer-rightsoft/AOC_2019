use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Can't read file");

    let mut items: Vec<usize> = input
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    items[1] = 12;
    items[2] = 2;

    let mut i = 0;

    while i < items.len() {
        let output_index = items[i + 3];
        let operand1_index = items[i + 1];
        let operand2_index = items[i + 2];

        match FromPrimitive::from_usize(items[i]) {
            Some(OpCodes::Add) => {
                items[output_index] = items[operand1_index] + items[operand2_index];
                i += 4;
            }
            Some(OpCodes::Multiply) => {
                items[output_index] = items[operand1_index] * items[operand2_index];
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

    println!("{}", items[0]);
}

#[derive(FromPrimitive)]
enum OpCodes {
    Add = 1,
    Multiply = 2,
    Halt = 99,
}
