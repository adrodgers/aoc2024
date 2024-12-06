fn main() {
    let input = include_str!("./input_1.txt");
    let output_1 = part_1(input);
    println!("{}", output_1);
    let output_2 = part_2(input);
    println!("{}", output_2);
}

fn part_1(input: &str) -> String {
    let word_search = parse_wordsearch(input);
    let y_len = word_search.len();
    let x_len = word_search[0].len();
    let mut sum = 0;
    for (i, line) in word_search.clone().into_iter().enumerate() {
        for (j, _char) in line.into_iter().enumerate() {
            if word_search[i][j] == 'X' {
                // find vertical, up
                if i >= 3
                    && word_search[i - 1][j] == 'M'
                    && word_search[i - 2][j] == 'A'
                    && word_search[i - 3][j] == 'S'
                {
                    sum += 1;
                }
                // find vertical, down
                if i < y_len - 3
                    && word_search[i + 1][j] == 'M'
                    && word_search[i + 2][j] == 'A'
                    && word_search[i + 3][j] == 'S'
                {
                    sum += 1;
                }
                // find horizontal, left
                if j >= 3
                    && word_search[i][j - 1] == 'M'
                    && word_search[i][j - 2] == 'A'
                    && word_search[i][j - 3] == 'S'
                {
                    sum += 1;
                }
                // find horizontal, right
                if j < x_len - 3
                    && word_search[i][j + 1] == 'M'
                    && word_search[i][j + 2] == 'A'
                    && word_search[i][j + 3] == 'S'
                {
                    sum += 1;
                }
                // find diagonal, right, up
                if i >= 3
                    && j < x_len - 3
                    && word_search[i - 1][j + 1] == 'M'
                    && word_search[i - 2][j + 2] == 'A'
                    && word_search[i - 3][j + 3] == 'S'
                {
                    sum += 1;
                }
                // find diagonal, right, down
                if i < y_len - 3
                    && j < x_len - 3
                    && word_search[i + 1][j + 1] == 'M'
                    && word_search[i + 2][j + 2] == 'A'
                    && word_search[i + 3][j + 3] == 'S'
                {
                    sum += 1;
                }
                // find diagonal, left, up
                if i >= 3
                    && j >= 3
                    && word_search[i - 1][j - 1] == 'M'
                    && word_search[i - 2][j - 2] == 'A'
                    && word_search[i - 3][j - 3] == 'S'
                {
                    sum += 1;
                }
                // find diagonal, left, down
                if i < y_len - 3
                    && j >= 3
                    && word_search[i + 1][j - 1] == 'M'
                    && word_search[i + 2][j - 2] == 'A'
                    && word_search[i + 3][j - 3] == 'S'
                {
                    sum += 1;
                }
            }
        }
    }
    sum.to_string()
}

fn part_2(input: &str) -> String {
    let word_search = parse_wordsearch(input);
    let y_len = word_search.len();
    let x_len = word_search[0].len();
    let mut sum = 0;
    for (i, line) in word_search.clone().into_iter().enumerate() {
        for (j, _char) in line.into_iter().enumerate() {
            if word_search[i][j] == 'A' {
                // find diagonal, NW - SE
                if i >= 1
                    && i < y_len - 1
                    && j < x_len - 1
                    && j >= 1
                    && ((word_search[i - 1][j - 1] == 'M' && word_search[i + 1][j + 1] == 'S')
                        || (word_search[i - 1][j - 1] == 'S' && word_search[i + 1][j + 1] == 'M'))
                {
                    // find diagonal, SW - NE
                    if (word_search[i + 1][j - 1] == 'M' && word_search[i - 1][j + 1] == 'S')
                        || (word_search[i + 1][j - 1] == 'S' && word_search[i - 1][j + 1] == 'M')
                    {
                        sum += 1;
                    }
                }
            }
        }
    }
    sum.to_string()
}

fn parse_wordsearch(input: &str) -> Vec<Vec<char>> {
    let mut word_search: Vec<Vec<char>> = vec![];
    input.lines().enumerate().for_each(|(i, l)| {
        word_search.push(vec![]);
        l.chars().for_each(|c| {
            if c == 'X' || c == 'M' || c == 'A' || c == 'S' {
                word_search[i].push(c)
            } else {
                word_search[i].push('.')
            }
        })
    });
    word_search
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let test_input = "....XXMAS.
.SAMXMS...
...S..A...
..A.A.MS.X
XMASAMX.MM
X.....XA.A
S.S.S.S.SS
.A.A.A.A.A
..M.M.M.MM
.X.X.XMASX
";
        let output = part_1(test_input);
        assert_eq!(output, "18".to_string())
    }
    #[test]
    fn test_2() {
        let test_input = ".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........
";
        let output = part_2(test_input);
        assert_eq!(output, "9".to_string())
    }
}
