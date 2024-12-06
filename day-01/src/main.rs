fn main() {
    let input = include_str!("./input_1.txt");
    let output_1 = part_1(input);
    println!("{}", output_1);
    let output_2 = part_2(input);
    println!("{}", output_2);
}

fn part_1(input: &str) -> String {
    let mut left_list: Vec<i32> = vec![];
    let mut right_list: Vec<i32> = vec![];
    for line in input.lines() {
        let (left_num, right_num) = parse_line(line);
        left_list.push(left_num);
        right_list.push(right_num);
    }
    left_list.sort();
    right_list.sort();
    // take absolute difference and sum
    let output: i32 = left_list
        .into_iter()
        .zip(right_list)
        .map(|(l, r)| (l - r).abs())
        .sum();
    output.to_string()
}

fn part_2(input: &str) -> String {
    let mut left_list: Vec<i32> = vec![];
    let mut right_list: Vec<i32> = vec![];
    for line in input.lines() {
        let (left_num, right_num) = parse_line(line);
        left_list.push(left_num);
        right_list.push(right_num);
    }
    // let mut similarity_map: HashMap<i32, i32> = HashMap::new();

    let output: i32 = left_list
        .into_iter()
        .map(|l| {
            let similarity = right_list.clone().into_iter().filter(|r| *r == l).count();
            similarity as i32 * l
        })
        .sum();
    output.to_string()
}

fn parse_line(line: &str) -> (i32, i32) {
    let mut nums = line.split_whitespace();
    let first = nums.next().unwrap();
    let second = nums.next().unwrap();
    (first.parse().unwrap(), second.parse().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let test_input = "3   4
4   3
2   5
1   3
3   9
3   3";
        let output = part_1(test_input);
        assert_eq!(output, "11".to_string())
    }
    #[test]
    fn test_2() {
        let test_input = "3   4
4   3
2   5
1   3
3   9
3   3";
        let output = part_2(test_input);
        assert_eq!(output, "31".to_string())
    }
}
