use std::{
    char::MAX,
    cmp,
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    let result = run("input.txt");
    println!("point: {:?} and current_distance: {} ", result.0, result.1);
}

fn run(path: &str) -> (Point, i64) {
    let input = fs::read_to_string(path).expect("file not found");
    let raw_directions: Vec<&str> = input.split('\n').collect();
    let wire_1_directions: Vec<Direction> = raw_directions[0].split(',').map(parse).collect();
    let wire_2_directions: Vec<Direction> = raw_directions[1].split(',').map(parse).collect();
    let mut wire_1_stops: HashSet<Point> = HashSet::new();
    let mut wire_2_stops: HashSet<Point> = HashSet::new();
    let mut wire_1_distances: HashMap<Point, i64> = HashMap::new();
    let mut wire_2_distances: HashMap<Point, i64> = HashMap::new();
    calculate_stops(&wire_1_directions, &mut wire_1_stops, &mut wire_1_distances);
    calculate_stops(&wire_2_directions, &mut wire_2_stops, &mut wire_2_distances);

    let mut intersections: Vec<Point> = Vec::new();
    for o in wire_1_stops {
        if wire_2_stops.contains(&o) {
            intersections.push(o);
        }
    }

    let mut min_distance = i64::MAX;
    let mut current_point: Point = Point { x: 0, y: 0 };

    for intersection in intersections {
        let current_distance = wire_1_distances[&intersection] + wire_2_distances[&intersection];

        if current_distance < min_distance {
            min_distance = current_distance;
            current_point = intersection;
        }
    }

    (current_point, min_distance)
}

fn calculate_stops(
    directions: &Vec<Direction>,
    stops: &mut HashSet<Point>,
    distances: &mut HashMap<Point, i64>,
) {
    let mut pos = Point { x: 0, y: 0 };
    let mut total_distance = 0;
    for d in directions {
        match d {
            Direction::X(d_x, direction) => {
                for _i in 0..d_x.to_owned() {
                    pos.x += direction;
                    total_distance += 1;
                    distances.insert(pos.clone(), total_distance);
                    stops.insert(pos.clone());
                }
            }
            Direction::Y(d_y, direction) => {
                for _i in 0..d_y.to_owned() {
                    pos.y += direction;
                    total_distance += 1;
                    distances.insert(pos.clone(), total_distance);
                    stops.insert(pos.clone());
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

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
}

#[cfg(test)]
mod tests {
    use crate::run;

    #[test]
    fn test_input1() {
        let result = run("test_input.txt");
        assert_eq!(result.1, 610);
    }

    #[test]
    fn test_input2() {
        let result = run("test_input3.txt");
        assert_eq!(result.1, 410);
    }

    #[test]
    fn test_input3() {
        let result = run("test_input2.txt");
        assert_eq!(result.1, 30);
    }
}
