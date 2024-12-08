fn main() {
    let input = include_str!("./input_1.txt");
    let output_1 = part_1(input);
    println!("{}", output_1);
    // let output_2 = part_2(input);
    // println!("{}", output_2);
}

fn part_1(input: &str) -> String {
    "".to_string()
}

fn part_2(input: &str) -> String {
    "".to_string()
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
    // #[test]
    // fn test_2() {
    //     let output = part_2(TEST_INPUT);
    //     assert_eq!(output, "11387".to_string())
    // }
}
