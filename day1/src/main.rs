use std::io;
use std::fs;

fn main() {
 
    let content = fs::read_to_string("day1Input.txt").expect("can't find the file");;
    
    let items = content.split("\n");
    
    let mut total_fuel = 0.0;
    for item in items {
        let value = (item.parse::<f32>().unwrap() / 3.0).floor();
        total_fuel = total_fuel + value - 2.0;
    }
    
    println!("{total_fuel}");
    
}
