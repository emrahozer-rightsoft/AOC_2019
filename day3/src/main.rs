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
            Direction::X(d_x) => {
                for _i in 0..d_x.abs().to_owned() {
                    if *d_x < 0 {
                        pos.x -= 1;
                    } else {
                        pos.x += 1;
                    }

                    stops.insert(pos);
                }
            }
            Direction::Y(d_y) => {
                for _i in 0..d_y.abs().to_owned() {
                    if *d_y < 0 {
                        pos.y -= 1;
                    } else {
                        pos.y += 1;
                    }
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
    let distance = s[1..].parse::<i64>().unwrap();
    //return
    match direction_char {
        'D' => Direction::Y(-distance),
        'U' => Direction::Y(distance),
        'L' => Direction::X(-distance),
        'R' => Direction::X(distance),
        _ => Direction::None,
    }
}

#[derive(Debug)]
enum Direction {
    Y(i64),
    X(i64),
    None,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point {
    x: i64,
    y: i64,
}
