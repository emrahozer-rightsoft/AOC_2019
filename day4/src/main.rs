use std::collections::{hash_map::Entry, HashMap};

fn main() {
    let mut suitable_numbers = 0;
    for i in 278384..824795 {
        if check(i) {
            suitable_numbers += 1;
        }
    }
    println!("total found {}", suitable_numbers);
}

fn check(value: i64) -> bool {
    let binding = value.to_string();

    let values = binding
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<u32>>();

    let mut hash_map: HashMap<u32, u32> = HashMap::new();

    for i in 0..values.len() {
        let value = values[i];
        if i > 0 && values[i - 1] > value {
            return false;
        }

        if let Entry::Vacant(e) = hash_map.entry(value) {
            e.insert(1);
        } else if let Some(count) = hash_map.get_mut(&value) {
            *count += 1;
        }
    }

    hash_map.values().any(|&x| x > 1)
    //hash_map.values().any(|&x| x == 2) for part 2
}
