use core::panic;
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    let input = include_str!("./input_1.txt");
    let output_1 = part_1(input);
    println!("{}", output_1);
    let output_2 = part_2(input);
    println!("{}", output_2);
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

const UP: (i32, i32) = (0, -1);
const DOWN: (i32, i32) = (0, 1);
const LEFT: (i32, i32) = (-1, 0);
const RIGHT: (i32, i32) = (1, 0);

fn part_1(input: &str) -> String {
    let mut map: HashMap<(i32, i32), char> = HashMap::new();
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut possible_regions: VecDeque<(i32, i32)> = VecDeque::new();
    input.lines().enumerate().for_each(|(j, l)| {
        l.chars().enumerate().for_each(|(i, c)| {
            map.insert((i as i32, j as i32), c);
        })
    });
    dbg!(&map);
    possible_regions.push_back((0, 0));
    // Stop when we have visited all map locations
    let mut total_cost = 0;
    while visited.len() != map.len() {
        if let Some(start) = possible_regions.pop_front() {
            let mut perimeter = 0;
            let mut area = 0;
            let region_char = map.get(&start).unwrap();
            dbg!(&region_char);
            let mut next_positions: VecDeque<(i32, i32)> = VecDeque::new();
            let mut perim_checked_this_region: HashSet<(i32, i32, i32, i32)> = HashSet::new();
            next_positions.push_back(start);
            if !visited.contains(&(start.0, start.1)) {
                // keep going until region fully explored
                while !next_positions.is_empty() {
                    if let Some(pos) = next_positions.pop_front() {
                        // dbg!(&pos);
                        if !visited.contains(&pos) {
                            area += 1;
                        }
                        for (i, j) in [UP, DOWN, LEFT, RIGHT] {
                            let tmp_pos = (pos.0 + i, pos.1 + j);
                            // skip if already visited this region
                            match map.get(&tmp_pos) {
                                Some(c) => {
                                    if c == region_char {
                                        if !visited.contains(&tmp_pos)
                                            && !next_positions.contains(&tmp_pos)
                                        {
                                            next_positions.push_back(tmp_pos);
                                        }
                                    } else if !perim_checked_this_region
                                        .contains(&(tmp_pos.0, tmp_pos.1, i, j))
                                    {
                                        perim_checked_this_region
                                            .insert((tmp_pos.0, tmp_pos.1, i, j));
                                        perimeter += 1;
                                        possible_regions.push_back(tmp_pos);
                                    }
                                }
                                None => {
                                    if !perim_checked_this_region
                                        .contains(&(tmp_pos.0, tmp_pos.1, i, j))
                                    {
                                        perimeter += 1;
                                    }
                                }
                            }
                        }
                        visited.insert(pos);
                    }
                }
                // dbg!(&perimeter);
                // dbg!(&area);
                total_cost += perimeter * area;
            };
        };
    }
    total_cost.to_string()
}

fn part_2(input: &str) -> String {
    let mut map: HashMap<(i32, i32), char> = HashMap::new();
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut possible_regions: VecDeque<(i32, i32)> = VecDeque::new();
    input.lines().enumerate().for_each(|(j, l)| {
        l.chars().enumerate().for_each(|(i, c)| {
            map.insert((i as i32, j as i32), c);
        })
    });
    possible_regions.push_back((0, 0));
    // Stop when we have visited all map locations
    let mut total_cost = 0;
    while visited.len() != map.len() {
        if let Some(start) = possible_regions.pop_front() {
            let mut perimeter = 0;
            let mut area = 0;
            let region_char = map.get(&start).unwrap();
            dbg!(&region_char);
            let mut next_positions: VecDeque<(i32, i32)> = VecDeque::new();
            let mut perim_checked_this_region: HashSet<(i32, i32, i32, i32)> = HashSet::new();
            next_positions.push_back(start);
            if !visited.contains(&(start.0, start.1)) {
                // keep going until region fully explored
                while !next_positions.is_empty() {
                    if let Some(pos) = next_positions.pop_front() {
                        dbg!(&pos);
                        if !visited.contains(&pos) {
                            area += 1;
                        }
                        for (i, j) in [UP, DOWN, LEFT, RIGHT] {
                            let tmp_pos = (pos.0 + i, pos.1 + j);
                            // skip if already visited this region
                            match map.get(&tmp_pos) {
                                Some(c) => {
                                    if c == region_char {
                                        if !visited.contains(&tmp_pos)
                                            && !next_positions.contains(&tmp_pos)
                                        {
                                            next_positions.push_back(tmp_pos);
                                        }
                                    } else if !perim_checked_this_region
                                        .contains(&(tmp_pos.0, tmp_pos.1, i, j))
                                    {
                                        perim_checked_this_region
                                            .insert((tmp_pos.0, tmp_pos.1, i, j));
                                        possible_regions.push_back(tmp_pos);
                                        perimeter += 1;
                                    }
                                }
                                None => {
                                    if !perim_checked_this_region
                                        .contains(&(tmp_pos.0, tmp_pos.1, i, j))
                                    {
                                        perim_checked_this_region
                                            .insert((tmp_pos.0, tmp_pos.1, i, j));
                                        perimeter += 1;
                                    }
                                }
                            }
                        }
                        visited.insert(pos);
                    }
                }
                // check all perim vales to determine the amount of sides
                // pick a point on the perimeter, traverse the perimeter until the start point is reached, track the number of direction changes.
                // NOPE
                // Find instances of corners and sum!
                // Internal corners are hardest, leave these until last, remaining should be internal
                dbg!(&perim_checked_this_region);
                let scaled: Vec<(i32, i32, Direction)> = perim_checked_this_region
                    .iter()
                    .map(|t| {
                        let dir = (t.2, t.3);
                        println!("{:?} : {:?} : {:?}", dir, (t.2, t.3), UP);
                        match dir {
                            UP => (t.0 + DOWN.0, t.1 + DOWN.1, Direction::Up),
                            DOWN => (t.0 + UP.0, t.1 + UP.1, Direction::Down),
                            LEFT => (t.0 + RIGHT.0, t.1 + RIGHT.1, Direction::Left),
                            RIGHT => (t.0 + LEFT.0, t.1 + LEFT.1, Direction::Right),
                            _ => panic!("ahhhh"),
                        }
                    })
                    .collect();
                dbg!(scaled);
                // let mut v = Vec::from_iter(perim_checked_this_region);
                // let mut up: Vec<(i32, i32, i32, i32)> =
                //     v.into_iter().filter(|p| (p.2, p.3) == UP).collect();
                // up.sort();
                // dbg!(up);
                total_cost += perimeter * area;
            };
        };
    }
    total_cost.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
";

    #[test]
    fn test_1() {
        let output = part_1(TEST_INPUT);
        assert_eq!(output, "1930".to_string())
    }

    #[test]
    fn test_case_1() {
        const INPUT: &str = "AAAA
BBCD
BBCC
EEEC
";
        let output = part_1(INPUT);
        assert_eq!(output, "140".to_string())
    }
    #[test]
    fn test_case_2() {
        const INPUT: &str = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO
";
        let output = part_1(INPUT);
        assert_eq!(output, "772".to_string())
    }

    #[test]
    fn test_2() {
        let output = part_2(TEST_INPUT);
        assert_eq!(output, "1206".to_string())
    }
    #[test]
    fn test_case_1_2() {
        const INPUT: &str = "AAAA
BBCD
BBCC
EEEC
";
        let output = part_2(INPUT);
        assert_eq!(output, "80".to_string())
    }

    #[test]
    fn test_case_2_2() {
        const INPUT: &str = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO
";
        let output = part_2(INPUT);
        assert_eq!(output, "436".to_string())
    }

    #[test]
    fn test_case_2_3() {
        const INPUT: &str = "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE
";
        let output = part_2(INPUT);
        assert_eq!(output, "236".to_string())
    }

    #[test]
    fn test_case_2_4() {
        const INPUT: &str = "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA
";
        let output = part_2(INPUT);
        assert_eq!(output, "1206".to_string())
    }
}
