fn main() {
    let input = include_str!("./input_1.txt");
    let output_1 = part_1(input);
    println!("{}", output_1);
    let output_2 = part_2(input);
    println!("{}", output_2);
}

fn part_1(input: &str) -> String {
    let chars = input.chars();
    let possible_valid: Vec<_> = input.match_indices("mul(").collect();

    let vec: Vec<String> = chars.map(|c| c.to_string()).collect();
    let output: i32 = possible_valid
        .into_iter()
        .map(|possible| {
            let start = possible.0;
            let mut valid_string = String::new();
            for i in 0..12 {
                if (start + i) < vec.len() - 1 {
                    valid_string += &vec[start + i];
                } else {
                    continue;
                }
            }
            if valid_string.contains(')') {
                let mut valid_trimmed = valid_string.split_inclusive(')');
                let vals: Vec<i32> = valid_trimmed
                    .next()
                    .unwrap()
                    .strip_prefix("mul(")
                    .unwrap()
                    .strip_suffix(")")
                    .unwrap()
                    .split(',')
                    .flat_map(|num| num.parse::<i32>())
                    .collect();
                println!("{:?}", vals);
                return vals[0] * vals[1];
            }
            0
        })
        .sum();

    output.to_string()
}

fn part_2(input: &str) -> String {
    let chars = input.chars();
    let vec: Vec<String> = chars.map(|c| c.to_string()).collect();
    let possible_valid: Vec<_> = input.match_indices("mul(").collect();
    let do_mul: Vec<_> = input.match_indices("do()").collect();
    println!("{:?}", do_mul);
    println!("{:?}", do_mul.len());

    let dont_mul: Vec<_> = input.match_indices("don't()").collect();
    println!("{:?}", dont_mul);
    println!("{:?}", dont_mul.len());

    let mut do_index_ranges: Vec<(usize, usize)> = vec![];
    do_index_ranges.push((0, dont_mul[0].0));
    do_mul.iter().for_each(|(i, _do_mul)| {
        if let Some(val) = dont_mul
            .iter()
            .filter(|(j, _)| j > i)
            .take(1)
            .map(|(j, _)| *j)
            .collect::<Vec<usize>>()
            .first()
        {
            do_index_ranges.push((*i, *val))
        } else {
            do_index_ranges.push((*i, vec.len() - 1))
        }
    });
    println!("{:?}", do_index_ranges);

    let output: i32 = possible_valid
        .into_iter()
        .map(|possible| {
            let start = possible.0;

            let mut valid_string = String::new();
            for i in 0..12 {
                if (start + i) < vec.len() - 1 {
                    valid_string += &vec[start + i];
                } else {
                    continue;
                }
            }
            if do_index_ranges
                .iter()
                .any(|(s, e)| s <= &start && e >= &start)
                && valid_string.contains(')')
            {
                let mut valid_trimmed = valid_string.split_inclusive(')');
                let vals: Vec<i32> = valid_trimmed
                    .next()
                    .unwrap()
                    .strip_prefix("mul(")
                    .unwrap()
                    .strip_suffix(")")
                    .unwrap()
                    .split(',')
                    .flat_map(|num| num.parse::<i32>())
                    .collect();
                return vals[0] * vals[1];
            };
            0
        })
        .sum();

    output.to_string()
}

// fn parse_line(line: &str) -> Vec<i16> {
//     let levels = line.split_whitespace();
//     levels.map(|l| l.parse().unwrap()).collect()
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let test_input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let output = part_1(test_input);
        assert_eq!(output, "161".to_string())
    }
    #[test]
    fn test_2() {
        let test_input =
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let output = part_2(test_input);
        assert_eq!(output, "48".to_string())
    }
}
