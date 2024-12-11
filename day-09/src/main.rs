fn main() {
    let input = include_str!("./input_1.txt");
    let output_1 = part_1(input);
    println!("{}", output_1);
    let output_2 = part_2(input);
    println!("{}", output_2);
}

fn part_1(input: &str) -> String {
    let mut memory: Vec<String> = vec![];
    input.trim().chars().enumerate().for_each(|(i, c)| {
        if i % 2 == 0 {
            std::iter::repeat((i / 2).to_string())
                .take(c.to_digit(10).unwrap() as usize)
                .for_each(|c| memory.push(c));
        } else {
            std::iter::repeat(".".to_string())
                .take(c.to_digit(10).unwrap() as usize)
                .for_each(|c| memory.push(c));
        }
    });
    while memory.contains(&'.'.to_string()) {
        let v = memory.iter().position(|c| c == &'.'.to_string()).unwrap();
        let n = memory.pop().unwrap();
        memory[v] = n;

        while memory.last() == Some(&'.'.to_string()) {
            memory.pop();
        }
    }
    memory
        .into_iter()
        .enumerate()
        .fold(0, |acc, (i, c)| acc + i as u64 * c.parse::<u64>().unwrap())
        .to_string()
}

fn part_2(input: &str) -> String {
    let _ = input;
    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "2333133121414131402";

    #[test]
    fn test_1() {
        let output = part_1(TEST_INPUT);
        assert_eq!(output, "1928".to_string())
    }
    // #[test]
    // fn test_2() {
    //     let output = part_2(TEST_INPUT);
    //     assert_eq!(output, "2858".to_string())
    // }
}
