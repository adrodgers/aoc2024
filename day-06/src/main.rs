use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("./input_1.txt");
    // let output_1 = part_1(input);
    // println!("{}", output_1);
    let output_2 = part_2(input);
    println!("{}", output_2);
}

type Position = (i32, i32);

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
enum Direction {
    East,
    North,
    South,
    West,
}

#[derive(Debug, Clone)]
struct GuardState {
    position: Position,
    direction: Direction,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum MapSpace {
    Empty,
    Obstacle,
}

fn part_1(input: &str) -> String {
    let mut map: HashMap<(i32, i32), MapSpace> = HashMap::new();
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut guard_state: Option<GuardState> = None;
    input.lines().enumerate().for_each(|(i, l)| {
        l.chars().enumerate().for_each(|(j, c)| {
            if c == '>' || c == '<' || c == '^' || c == 'v' {
                let dir: Option<Direction> = match c {
                    '>' => Some(Direction::East),
                    '^' => Some(Direction::North),
                    'v' => Some(Direction::South),
                    '<' => Some(Direction::West),
                    _ => None,
                };
                guard_state = Some(GuardState {
                    position: (i as i32, j as i32),
                    direction: dir.unwrap(),
                });
                visited.insert((i as i32, j as i32));
                map.insert((i as i32, j as i32), MapSpace::Empty);
            } else if c == '#' {
                map.insert((i as i32, j as i32), MapSpace::Obstacle);
            } else {
                map.insert((i as i32, j as i32), MapSpace::Empty);
            }
        })
    });
    // println!("{:?}", map);
    // println!("{:?}", visited);
    // println!("{:?}", guard_state);
    loop {
        if let Some(state) = guard_state.clone() {
            let next_position: Position = match state.direction {
                Direction::North => (state.position.0 - 1, state.position.1),
                Direction::East => (state.position.0, state.position.1 + 1),
                Direction::South => (state.position.0 + 1, state.position.1),
                Direction::West => (state.position.0, state.position.1 - 1),
            };
            if let Some(space) = map.get(&next_position) {
                match space {
                    MapSpace::Empty => {
                        visited.insert(next_position);
                        guard_state.as_mut().unwrap().position = next_position;
                    }
                    MapSpace::Obstacle => {
                        guard_state.as_mut().unwrap().direction = match state.direction {
                            Direction::North => Direction::East,
                            Direction::East => Direction::South,
                            Direction::South => Direction::West,
                            Direction::West => Direction::North,
                        };
                    }
                }
            } else {
                break;
            }
        }
        // println!("{:?}", visited.len());
    }
    visited.len().to_string()
}

fn part_2(input: &str) -> String {
    let mut map: HashMap<(i32, i32), MapSpace> = HashMap::new();
    let mut visited: HashSet<(i32, i32, Direction)> = HashSet::new();
    let mut guard_state: Option<GuardState> = None;
    input.lines().enumerate().for_each(|(i, l)| {
        l.chars().enumerate().for_each(|(j, c)| {
            if c == '>' || c == '<' || c == '^' || c == 'v' {
                let dir: Option<Direction> = match c {
                    '>' => Some(Direction::East),
                    '^' => Some(Direction::North),
                    'v' => Some(Direction::South),
                    '<' => Some(Direction::West),
                    _ => None,
                };
                guard_state = Some(GuardState {
                    position: (i as i32, j as i32),
                    direction: dir.clone().unwrap(),
                });
                visited.insert((i as i32, j as i32, dir.clone().unwrap()));
                map.insert((i as i32, j as i32), MapSpace::Empty);
            } else if c == '#' {
                map.insert((i as i32, j as i32), MapSpace::Obstacle);
            } else {
                map.insert((i as i32, j as i32), MapSpace::Empty);
            }
        })
    });
    let initial_guard_state = guard_state.clone();
    // println!("{:?}", map);
    // println!("{:?}", visited);
    // println!("{:?}", guard_state);
    let mut added_obstacles = 0;
    for ((i, j), space) in map.clone() {
        println!("{:?}", added_obstacles);
        if space == MapSpace::Empty {
            let mut tmp_map = map.clone();
            if let Some(x) = tmp_map.get_mut(&(i, j)) {
                *x = MapSpace::Obstacle;
                // println!("{:?}", tmp_map.get(&(i, j)).unwrap());
            };
            guard_state = initial_guard_state.clone();
            visited.clear();
            visited.insert((
                initial_guard_state.as_ref().unwrap().position.0,
                initial_guard_state.as_ref().unwrap().position.1,
                initial_guard_state.as_ref().unwrap().direction.clone(),
            ));
            loop {
                if let Some(state) = guard_state.clone() {
                    let next_position: Position = match state.direction {
                        Direction::North => (state.position.0 - 1, state.position.1),
                        Direction::East => (state.position.0, state.position.1 + 1),
                        Direction::South => (state.position.0 + 1, state.position.1),
                        Direction::West => (state.position.0, state.position.1 - 1),
                    };
                    if let Some(space) = tmp_map.get(&next_position) {
                        match space {
                            MapSpace::Empty => {
                                let success = visited.insert((
                                    next_position.0,
                                    next_position.1,
                                    state.direction,
                                ));
                                if !success {
                                    added_obstacles += 1;
                                    break;
                                }
                                guard_state.as_mut().unwrap().position = next_position;
                            }
                            MapSpace::Obstacle => {
                                let dir = match state.direction {
                                    Direction::North => Direction::East,
                                    Direction::East => Direction::South,
                                    Direction::South => Direction::West,
                                    Direction::West => Direction::North,
                                };
                                guard_state.as_mut().unwrap().direction = dir.clone();
                                let success =
                                    visited.insert((state.position.0, state.position.1, dir));
                                if !success {
                                    added_obstacles += 1;
                                    break;
                                }
                            }
                        }
                    } else {
                        break;
                    }
                }
                // println!("{:?}", visited.len());
            }
        }
    }
    added_obstacles.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

    // #[test]
    // fn test_1() {
    //     let output = part_1(TEST_INPUT);
    //     assert_eq!(output, "41".to_string())
    // }
    #[test]
    fn test_2() {
        let output = part_2(TEST_INPUT);
        assert_eq!(output, "6".to_string())
    }
}
