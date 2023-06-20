use std::fs;

fn main() {
    let content = fs::read_to_string("day1Input.txt").expect("can't find the file");
    let items = content.split('\n');

    let mut total_fuel = 0.0;
    for item in items {
        let weight = item.parse::<f32>().unwrap();
        let mut fuel = calculate(weight);

        while fuel > 0.0 {
            total_fuel += fuel;
            fuel = calculate(fuel);
        }
    }
    println!("{total_fuel}");
}

fn calculate(fuel: f32) -> f32 {
    (fuel / 3.0).floor() - 2.0
}
