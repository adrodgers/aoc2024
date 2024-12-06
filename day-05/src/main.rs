fn main() {
    let input = include_str!("./input_1.txt");
    let output_1 = part_1(input);
    println!("{}", output_1);
    let output_2 = part_2(input);
    println!("{}", output_2);
}

type Rules = (i32, i32);

fn part_1(input: &str) -> String {
    let parts: Vec<&str> = input.split("\n\n").collect();
    // Parse order rules
    let rules: Vec<Rules> = parts
        .first()
        .unwrap()
        .lines()
        .map(|l| {
            let v: Vec<i32> = l
                .split("|")
                .map(|val| val.parse::<i32>().unwrap())
                .collect();
            (v.first().unwrap().to_owned(), v.last().unwrap().to_owned())
        })
        .collect();
    let pages: Vec<Vec<i32>> = parts
        .last()
        .unwrap()
        .lines()
        .map(|l| {
            l.split(',')
                .map(|val| val.parse::<i32>().unwrap())
                .collect()
        })
        .collect();
    println!("{:?}", rules);
    println!("{:?}", pages);
    let mut output = 0;
    for page in pages {
        println!("{:?}", page);
        let mut relevant_rules: Vec<Rules> = vec![];
        for (before, after) in rules.clone() {
            if page.contains(&before) && page.contains(&after) {
                relevant_rules.push((before, after))
            }
        }
        println!("{:?}", relevant_rules);
        let mut should_sum = true;
        for (before, after) in relevant_rules {
            if page.clone().into_iter().position(|v| v == before).unwrap()
                > page.clone().into_iter().position(|v| v == after).unwrap()
            {
                should_sum = false;
                break;
            }
        }
        if should_sum {
            println!("{:?}", page[(page.len() - 1) / 2]);
            output += page[(page.len() - 1) / 2];
        }
    }
    output.to_string()
}

fn part_2(input: &str) -> String {
    let parts: Vec<&str> = input.split("\n\n").collect();
    let rules: Vec<Rules> = parts
        .first()
        .unwrap()
        .lines()
        .map(|l| {
            let v: Vec<i32> = l
                .split("|")
                .map(|val| val.parse::<i32>().unwrap())
                .collect();
            (v.first().unwrap().to_owned(), v.last().unwrap().to_owned())
        })
        .collect();
    let pages: Vec<Vec<i32>> = parts
        .last()
        .unwrap()
        .lines()
        .map(|l| {
            l.split(',')
                .map(|val| val.parse::<i32>().unwrap())
                .collect()
        })
        .collect();
    println!("{:?}", rules);
    println!("{:?}", pages);
    let mut output = 0;
    for page in pages {
        println!("{:?}", page);
        // Find the rules relevant to this page
        let mut relevant_rules: Vec<Rules> = vec![];
        for (before, after) in rules.clone() {
            if page.contains(&before) && page.contains(&after) {
                relevant_rules.push((before, after))
            }
        }
        // println!("{:?}", relevant_rules);
        // Check each relevant rule
        let mut broken_rules: Vec<Rules> = vec![];
        for (before, after) in relevant_rules.clone() {
            if page.clone().into_iter().position(|v| v == before).unwrap()
                > page.clone().into_iter().position(|v| v == after).unwrap()
            {
                broken_rules.push((before, after));
            }
        }
        if broken_rules.is_empty() {
            continue;
        }
        let mut tmp_page = page.clone();
        if !broken_rules.is_empty() {
            while !broken_rules.is_empty() {
                println!("{:?}", broken_rules);
                if !broken_rules.is_empty() {
                    let (before, after) = broken_rules[0];
                    let before_index = tmp_page
                        .clone()
                        .into_iter()
                        .position(|v| v == before)
                        .unwrap();
                    let after_index = tmp_page
                        .clone()
                        .into_iter()
                        .position(|v| v == after)
                        .unwrap();
                    tmp_page[before_index] = after;
                    tmp_page[after_index] = before;
                }
                println!("{:?}", tmp_page);
                broken_rules.clear();
                for (before, after) in relevant_rules.clone() {
                    if tmp_page
                        .clone()
                        .into_iter()
                        .position(|v| v == before)
                        .unwrap()
                        > tmp_page
                            .clone()
                            .into_iter()
                            .position(|v| v == after)
                            .unwrap()
                    {
                        broken_rules.push((before, after));
                    }
                }
                println!("{:?}", broken_rules);
            }
            println!("{:?}", tmp_page[(tmp_page.len() - 1) / 2]);
            output += tmp_page[(tmp_page.len() - 1) / 2];
        }
    }
    output.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

    #[test]
    fn test_1() {
        let output = part_1(TEST_INPUT);
        assert_eq!(output, "143".to_string())
    }
    #[test]
    fn test_2() {
        let output = part_2(TEST_INPUT);
        assert_eq!(output, "123".to_string())
    }
}
