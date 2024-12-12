use std::collections::HashMap;

fn main() {
    let input = include_str!("./input_1.txt");
    let output_1 = part_1(input, 25);
    println!("{}", output_1);
    let output_2 = part_2(input, 75);
    println!("{}", output_2);
}

fn part_1(input: &str, blinks: u128) -> String {
    let mut nums: Vec<u128> = input
        .lines()
        .next()
        .unwrap()
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();

    for b in 0..blinks {
        let mut tmp_nums: Vec<u128> = Vec::new();
        for num in nums.clone().into_iter() {
            if num == 0 {
                tmp_nums.push(1);
                continue;
            }
            // even splits into two
            let num_string = num.to_string();
            if num_string.len() % 2 == 0 {
                let split = num_string.split_at(num_string.len() / 2);
                tmp_nums.push(split.0.parse().unwrap());
                tmp_nums.push(split.1.parse().unwrap());
                continue;
            }
            // odd
            if num_string.len() % 2 != 0 {
                tmp_nums.push(num * 2024);
            }
        }
        println!("blink: {:?}, num stones: {:?}", b, tmp_nums.len());
        nums = tmp_nums;
    }
    nums.len().to_string()
}

// Part 1 solution will consume too much memory/take too long to run.
// The order is not important, just need to keep track of the stone numbers and the count in a map.
fn part_2(input: &str, blinks: u128) -> String {
    let mut nums: Vec<u128> = input
        .lines()
        .next()
        .unwrap()
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();

    let mut cache: HashMap<u128, u128> = HashMap::new();
    for num in nums.iter() {
        cache.entry(*num).and_modify(|v| *v += 1).or_insert(1);
    }
    dbg!(&cache);

    for _ in 0..blinks {
        let mut new_cache: HashMap<u128, u128> = HashMap::new();
        for (num, count) in cache.into_iter() {
            let num_string = num.to_string();
            dbg!(num);
            if num == 0 {
                new_cache
                    .entry(1)
                    .and_modify(|v| *v += count)
                    .or_insert(count);
                dbg!("==0");
            } else if num_string.len() % 2 == 0 {
                dbg!("%2 == 0");
                let split = num_string.split_at(num_string.len() / 2);
                new_cache
                    .entry(split.0.parse().unwrap())
                    .and_modify(|v| *v += count)
                    .or_insert(count);
                new_cache
                    .entry(split.1.parse().unwrap())
                    .and_modify(|v| *v += count)
                    .or_insert(count);
            } else {
                dbg!("else");
                new_cache
                    .entry(num * 2024)
                    .and_modify(|v| *v += count)
                    .or_insert(count);
            }
            // even splits into two
        }
        cache = new_cache;
        dbg!(&cache);
    }
    let output: u128 = cache.into_iter().map(|(k, v)| v).sum();
    output.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1_blink() {
        const INPUT: &str = "0 1 10 99 999";
        let output = part_1(INPUT, 1);
        assert_eq!(output, "7".to_string())
    }

    #[test]
    fn test_6_blink() {
        const INPUT: &str = "125 17";
        let output = part_1(INPUT, 6);
        assert_eq!(output, "22".to_string())
    }

    #[test]
    fn test_2_6_blink() {
        const INPUT: &str = "125 17";
        let output = part_2(INPUT, 6);
        assert_eq!(output, "22".to_string())
    }

    #[test]
    fn test_25_blink() {
        const INPUT: &str = "125 17";
        let output = part_1(INPUT, 25);
        assert_eq!(output, "55312".to_string())
    }

    #[test]
    fn test_2() {
        const INPUT: &str = "125 17";
        let output = part_2(INPUT, 25);
        assert_eq!(output, "55312".to_string())
    }
}
