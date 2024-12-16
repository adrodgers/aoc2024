use core::panic;
use std::collections::HashMap;

use itertools::Itertools;

fn main() {
    let input = include_str!("./input_1.txt");
    let output_1 = part_1(input);
    println!("{}", output_1);
    let output_2 = part_2(input);
    println!("{}", output_2);
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Space {
    Box,
    Wall,
    Robot,
}

const UP: (i32, i32) = (0, -1);
const DOWN: (i32, i32) = (0, 1);
const LEFT: (i32, i32) = (-1, 0);
const RIGHT: (i32, i32) = (1, 0);

fn part_1(input: &str) -> String {
    let mut map: HashMap<(i32, i32), Space> = HashMap::new();
    let mut robot_position: (i32, i32) = (0, 0);
    let mut instructions: Vec<(i32, i32)> = vec![];
    let (map_str, instructions_str) = input.split("\n\n").collect_tuple().unwrap();
    map_str.lines().enumerate().for_each(|(j, l)| {
        l.chars().enumerate().for_each(|(i, c)| {
            let space = match c {
                '#' => Some(Space::Wall),
                'O' => Some(Space::Box),
                '@' => Some(Space::Robot),
                '.' => None,
                _ => panic!(),
            };
            if let Some(s) = space {
                match s {
                    Space::Robot => robot_position = (i as i32, j as i32),
                    _ => {
                        map.insert((i as i32, j as i32), s);
                    }
                }
            }
        });
    });
    // dbg!(&map);
    instructions_str.lines().for_each(|l| {
        l.chars().for_each(|c| {
            let direction = match c {
                '^' => UP,
                'v' => DOWN,
                '<' => LEFT,
                '>' => RIGHT,
                _ => {
                    panic!()
                }
            };
            instructions.push(direction);
        })
    });
    // dbg!(&instructions);
    for instruction in instructions {
        // dbg!(&robot_position);
        // for space in map.iter() {
        //     if space.1 == &Space::Box {
        //         dbg!(space.0);
        //     }
        // }
        let next_pos = (
            robot_position.0 + instruction.0,
            robot_position.1 + instruction.1,
        );
        let mut spaces: Vec<((i32, i32), Space)> = vec![];
        let mut tmp_pos = next_pos;
        while let Some(pos) = map.get(&tmp_pos) {
            spaces.push((tmp_pos, pos.to_owned()));
            if pos == &Space::Wall {
                break;
            }
            tmp_pos.0 += instruction.0;
            tmp_pos.1 += instruction.1;
        }
        if spaces.is_empty() {
            robot_position = next_pos;
            continue;
        }
        if spaces.last().unwrap().1 == Space::Wall {
            continue;
        }
        map.remove(&spaces.first().unwrap().0);
        let mut to_add = spaces.last().unwrap().0;
        to_add.0 += instruction.0;
        to_add.1 += instruction.1;
        map.insert(to_add, Space::Box);
        robot_position = next_pos;
    }
    let mut output = 0;
    for space in map {
        if space.1 == Space::Box {
            output += 100 * space.0 .1 + space.0 .0;
        }
    }
    output.to_string()
}

fn part_2(input: &str) -> String {
    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<
";

    #[test]
    fn test_1() {
        let output = part_1(TEST_INPUT);
        assert_eq!(output, "2028".to_string())
    }

    #[test]
    fn test_1_large() {
        const TEST_INPUT_LARGE: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
";
        let output = part_1(TEST_INPUT_LARGE);
        assert_eq!(output, "10092".to_string())
    }

    #[test]
    fn test_2() {
        let output = part_2(TEST_INPUT);
        assert_eq!(output, "".to_string())
    }
}
