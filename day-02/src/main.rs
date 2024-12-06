fn main() {
    let input = include_str!("./input_1.txt");
    let output_1 = part_1(input);
    println!("{}", output_1);
    let output_2 = part_2(input);
    println!("{}", output_2);
}

fn part_1(input: &str) -> String {
    let mut records: Vec<Vec<i16>> = vec![];
    for line in input.lines() {
        records.push(parse_line(line));
    }
    println!("{:?}", records);
    let output = records.into_iter().filter(report_is_safe).count();
    output.to_string()
}

fn report_is_safe(report: &Vec<i16>) -> bool {
    let mut all_descending = true;
    let mut all_ascending = true;

    for (i, level) in report.clone().into_iter().enumerate() {
        if i > 0 {
            let diff = level - report[i - 1];
            if diff.abs() > 3 || diff == 0 {
                all_ascending = false;
                all_descending = false;
                break;
            };
            if diff < 0 {
                all_ascending = false;
            } else if diff > 0 {
                all_descending = false;
            };
        }
    }
    println!("{:?}", all_ascending);
    println!("{:?}", all_descending);
    all_ascending || all_descending
}

fn dampener(report: &Vec<i16>) -> bool {
    for i in 0..(report.len()) {
        let mut temp_report = report.clone();
        let _ = temp_report.remove(i);
        println!("{:?}", temp_report);
        if report_is_safe(&temp_report) {
            return true;
        }
    }
    false
}

fn part_2(input: &str) -> String {
    let mut records: Vec<Vec<i16>> = vec![];
    for line in input.lines() {
        records.push(parse_line(line));
    }
    println!("{:?}", records);
    let safe_reports = records.clone().into_iter().filter(report_is_safe);
    let unsafe_reports: Vec<Vec<i16>> =
        records.into_iter().filter(|r| !report_is_safe(r)).collect();
    let newly_safe = unsafe_reports.into_iter().filter(|ur| dampener(&ur));
    let output = safe_reports.count() + newly_safe.count();
    output.to_string()
}

fn parse_line(line: &str) -> Vec<i16> {
    let levels = line.split_whitespace();
    levels.map(|l| l.parse().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let test_input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";
        let output = part_1(test_input);
        assert_eq!(output, "2".to_string())
    }
    #[test]
    fn test_2() {
        let test_input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";
        let output = part_2(test_input);
        assert_eq!(output, "4".to_string())
    }
}
