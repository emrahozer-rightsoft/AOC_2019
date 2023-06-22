use std::{collections::HashSet, fs};

fn main() {
    let input = fs::read_to_string("input.txt").expect("file not found");
    let raw_directions: Vec<&str> = input.split('\n').collect();
    let wire_1_directions: Vec<Direction> = raw_directions[0].split(',').map(parse).collect();
    let wire_2_directions: Vec<Direction> = raw_directions[1].split(',').map(parse).collect();
    let mut wire_1_stops: HashSet<Point> = HashSet::new();
    let mut wire_2_stops: HashSet<Point> = HashSet::new();
    calculate_stops(&wire_1_directions, &mut wire_1_stops);
    calculate_stops(&wire_2_directions, &mut wire_2_stops);

    let mut intersections: Vec<Point> = Vec::new();
    for o in wire_1_stops {
        if wire_2_stops.contains(&o) {
            intersections.push(o);
        }
    }

    let mut min_distance = i64::MAX;
    let mut current_point: Point = Point { x: 0, y: 0 };

    for intersection in intersections {
        let current_distance = intersection.x.abs() + intersection.y.abs();

        if current_distance < min_distance {
            min_distance = current_distance;
            current_point = intersection;
        }
    }

    println!(
        "point: {:?} and current_distance: {} ",
        current_point, min_distance
    );
}
fn calculate_stops(directions: &Vec<Direction>, stops: &mut HashSet<Point>) {
    let mut pos = Point { x: 0, y: 0 };
    for d in directions {
        match d {
            Direction::X(d_x, direction) => {
                for _i in 0..d_x.to_owned() {
                    pos.x += direction;
                    stops.insert(pos);
                }
            }
            Direction::Y(d_y, direction) => {
                for _i in 0..d_y.to_owned() {
                    pos.y += direction;
                    stops.insert(pos);
                }
            }
            Direction::None => println!("error"),
        }
    }
}

fn parse(s: &str) -> Direction {
    let direction_char = s.chars().next().unwrap();
    //get the remaining characters
    let distance = s[1..].parse::<u64>().unwrap();
    //return
    match direction_char {
        'D' => Direction::Y(distance, -1),
        'U' => Direction::Y(distance, 1),
        'L' => Direction::X(distance, -1),
        'R' => Direction::X(distance, 1),
        _ => Direction::None,
    }
}

#[derive(Debug)]
enum Direction {
    Y(u64, i64),
    X(u64, i64),
    None,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point {
    x: i64,
    y: i64,
}
