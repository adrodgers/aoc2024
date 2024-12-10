use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    let input = include_str!("./input_1.txt");
    // let output_1 = part_1(input);
    // println!("{}", output_1);
    let output_2 = part_2(input);
    println!("{}", output_2);
}

const UP: (i32, i32) = (0, 1);
const DOWN: (i32, i32) = (0, -1);
const LEFT: (i32, i32) = (-1, 0);
const RIGHT: (i32, i32) = (1, 0);

fn part_1(input: &str) -> String {
    let mut map: HashMap<(i32, i32), u32> = HashMap::new();
    let mut trailheads: Vec<(i32, i32)> = Vec::new();
    let mut possible_paths: VecDeque<Vec<(i32, i32)>> = VecDeque::new();
    input.lines().enumerate().for_each(|(j, l)| {
        l.chars().enumerate().for_each(|(i, c)| {
            if let Some(height) = c.to_digit(10) {
                if height == 0 {
                    trailheads.push((i as i32, j as i32));
                };
                map.insert((i as i32, j as i32), height);
            };
        })
    });
    println!("{:?}", map);
    let mut score = 0;
    for trailhead in trailheads.iter() {
        let mut valid_paths: HashSet<(i32, i32)> = HashSet::new();
        possible_paths.push_back(vec![*trailhead]);
        while !possible_paths.is_empty() {
            let path = possible_paths.pop_back().unwrap();
            println!("{:?}", path);
            let current_pos = path.last().unwrap().to_owned();
            let current_height = map.get(&current_pos).unwrap().to_owned();
            for next_step in [UP, DOWN, LEFT, RIGHT] {
                let next_pos = (current_pos.0 + next_step.0, current_pos.1 + next_step.1);
                if let Some(next_height) = map.get(&next_pos) {
                    if *next_height == (current_height + 1) {
                        if *next_height == 9 && current_height == 8 {
                            valid_paths.insert(next_pos);
                            continue;
                        }
                        let mut extended_path = path.clone();
                        extended_path.push(next_pos);
                        possible_paths.push_front(extended_path);
                    }
                    continue;
                };
            }
        }
        score += valid_paths.len() as i32;
    }
    score.to_string()
}

fn part_2(input: &str) -> String {
    let mut map: HashMap<(i32, i32), u32> = HashMap::new();
    let mut trailheads: Vec<(i32, i32)> = Vec::new();
    let mut possible_paths: VecDeque<Vec<(i32, i32)>> = VecDeque::new();
    input.lines().enumerate().for_each(|(j, l)| {
        l.chars().enumerate().for_each(|(i, c)| {
            if let Some(height) = c.to_digit(10) {
                if height == 0 {
                    trailheads.push((i as i32, j as i32));
                };
                map.insert((i as i32, j as i32), height);
            };
        })
    });
    println!("{:?}", map);
    let mut score = 0;
    for trailhead in trailheads.iter() {
        possible_paths.push_back(vec![*trailhead]);
        while !possible_paths.is_empty() {
            let path = possible_paths.pop_back().unwrap();
            println!("{:?}", path);
            let current_pos = path.last().unwrap().to_owned();
            let current_height = map.get(&current_pos).unwrap().to_owned();
            for next_step in [UP, DOWN, LEFT, RIGHT] {
                let next_pos = (current_pos.0 + next_step.0, current_pos.1 + next_step.1);
                if let Some(next_height) = map.get(&next_pos) {
                    if *next_height == (current_height + 1) {
                        if *next_height == 9 && current_height == 8 {
                            score += 1;
                            continue;
                        }
                        let mut extended_path = path.clone();
                        extended_path.push(next_pos);
                        possible_paths.push_back(extended_path);
                    }
                };
            }
        }
        println!("{:?}", score);
        // score += valid_paths.len() as i32;
    }
    score.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";

    #[test]
    fn test_1() {
        let output = part_1(TEST_INPUT);
        assert_eq!(output, "36".to_string())
    }

    #[test]
    fn test_2() {
        let output = part_2(TEST_INPUT);
        assert_eq!(output, "81".to_string())
    }

    #[test]
    fn test_path() {
        let input: &str = "...0...
    ...1...
    ...2...
    6543456
    7.....7
    8.....8
    9.....9
    ";
        let output = part_1(input);
        assert_eq!(output, "2".to_string())
    }

    #[test]
    fn test_path_2() {
        let input: &str = "..90..9
...1.98
...2..7
6543456
765.987
876....
987....";
        let output = part_2(input);
        assert_eq!(output, "13".to_string())
    }

    #[test]
    fn test_path_3() {
        let input: &str = ".....0.
..4321.
..5..2.
..6543.
..7..4.
..8765.
..9....";
        let output = part_2(input);
        assert_eq!(output, "3".to_string())
    }
}
