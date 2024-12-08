use std::collections::{HashMap, HashSet};

use itertools::Itertools;

fn main() {
    let input = include_str!("./input_1.txt");
    let output_1 = part_1(input);
    println!("{}", output_1);
    let output_2 = part_2(input);
    println!("{}", output_2);
}

fn part_1(input: &str) -> String {
    let mut antennas: HashSet<(i32, i32, char)> = HashSet::new();
    let mut anti_nodes: HashSet<(i32, i32, char)> = HashSet::new();
    let mut unique_freqs: HashSet<char> = HashSet::new();
    let map_y_bound = input.lines().collect_vec().len() as i32 - 1;
    let map_x_bound = input.lines().collect_vec()[0].len() as i32 - 1;
    input.lines().enumerate().for_each(|(y_coord, l)| {
        l.chars()
            .enumerate()
            .filter(|(j, c)| c.is_alphanumeric())
            .map(|(x_coord, c)| (x_coord as i32, y_coord as i32, c))
            .for_each(|s| {
                antennas.insert(s);
                unique_freqs.insert(s.2);
            });
    });
    for freq in unique_freqs {
        let antennas_subset: HashSet<(i32, i32, char)> = antennas
            .clone()
            .into_iter()
            .filter(|a| a.2 == freq)
            .collect();
        for antenna in antennas_subset.clone() {
            for other_antenna in antennas_subset.clone() {
                // skip case of same antenna
                if other_antenna != antenna {
                    let x_coord = other_antenna.0 + (other_antenna.0 - antenna.0);
                    let y_coord = other_antenna.1 + (other_antenna.1 - antenna.1);
                    // is inside grid
                    if x_coord >= 0
                        && x_coord <= map_x_bound
                        && y_coord >= 0
                        && y_coord <= map_y_bound
                        && anti_nodes
                            .clone()
                            .into_iter()
                            .filter(|an| an.0 == x_coord && an.1 == y_coord)
                            .count()
                            == 0
                    {
                        anti_nodes.insert((x_coord, y_coord, antenna.2));
                    }
                }
            }
        }
    }
    println!("{:?}", anti_nodes);
    anti_nodes.len().to_string()
}

fn part_2(input: &str) -> String {
    let mut antennas: HashSet<(i32, i32, char)> = HashSet::new();
    let mut anti_nodes: HashSet<(i32, i32, char)> = HashSet::new();
    let mut unique_freqs: HashSet<char> = HashSet::new();
    let map_y_bound = input.lines().collect_vec().len() as i32 - 1;
    let map_x_bound = input.lines().collect_vec()[0].len() as i32 - 1;
    input.lines().enumerate().for_each(|(y_coord, l)| {
        l.chars()
            .enumerate()
            .filter(|(j, c)| c.is_alphanumeric())
            .map(|(x_coord, c)| (x_coord as i32, y_coord as i32, c))
            .for_each(|s| {
                antennas.insert(s);
                unique_freqs.insert(s.2);
            });
    });
    for freq in unique_freqs {
        let antennas_subset: HashSet<(i32, i32, char)> = antennas
            .clone()
            .into_iter()
            .filter(|a| a.2 == freq)
            .collect();
        for antenna in antennas_subset.clone() {
            for other_antenna in antennas_subset.clone() {
                // skip case of same antenna
                if other_antenna != antenna {
                    let x_diff = other_antenna.0 - antenna.0;
                    let y_diff = other_antenna.1 - antenna.1;
                    let mut x_coord = other_antenna.0;
                    let mut y_coord = other_antenna.1;
                    // is inside grid
                    while x_coord >= 0
                        && x_coord <= map_x_bound
                        && y_coord >= 0
                        && y_coord <= map_y_bound
                    {
                        if anti_nodes
                            .clone()
                            .into_iter()
                            .filter(|an| an.0 == x_coord && an.1 == y_coord)
                            .count()
                            == 0
                        {
                            anti_nodes.insert((x_coord, y_coord, antenna.2));
                        }
                        x_coord += x_diff;
                        y_coord += y_diff;
                    }
                }
            }
        }
    }
    println!("{:?}", anti_nodes);
    anti_nodes.len().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";

    #[test]
    fn test_1() {
        let output = part_1(TEST_INPUT);
        assert_eq!(output, "14".to_string())
    }
    #[test]
    fn test_2() {
        let output = part_2(TEST_INPUT);
        assert_eq!(output, "34".to_string())
    }
}
