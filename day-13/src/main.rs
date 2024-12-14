fn main() {
    let input = include_str!("./input_1.txt");
    let output_1 = part_1(input);
    println!("{}", output_1);
    let output_2 = part_2(input);
    println!("{}", output_2);
}

#[derive(Default, Debug)]
struct Game {
    button_a: (i64, i64),
    button_b: (i64, i64),
    prize: (i64, i64),
}

fn part_1(input: &str) -> String {
    let mut games: Vec<Game> = vec![];
    input.split("\n\n").for_each(|g| {
        let mut game = Game::default();
        g.lines().enumerate().for_each(|(i, l)| {
            l.split(":")
                .nth(1)
                .unwrap()
                .trim()
                .split(",")
                .enumerate()
                .for_each(|(j, v)| {
                    match i {
                        0 => match j {
                            0 => {
                                game.button_a.0 =
                                    v.split("+").nth(1).unwrap().parse::<i64>().unwrap();
                            }
                            1 => {
                                game.button_a.1 =
                                    v.split("+").nth(1).unwrap().parse::<i64>().unwrap();
                            }
                            _ => {
                                panic!("Shit")
                            }
                        },
                        1 => match j {
                            0 => {
                                game.button_b.0 =
                                    v.split("+").nth(1).unwrap().parse::<i64>().unwrap();
                            }
                            1 => {
                                game.button_b.1 =
                                    v.split("+").nth(1).unwrap().parse::<i64>().unwrap();
                            }
                            _ => {
                                panic!("Shit")
                            }
                        },
                        2 => match j {
                            0 => {
                                game.prize.0 = v.split("=").nth(1).unwrap().parse::<i64>().unwrap();
                            }
                            1 => {
                                game.prize.1 = v.split("=").nth(1).unwrap().parse::<i64>().unwrap();
                            }
                            _ => {
                                panic!("Shit")
                            }
                        },
                        _ => {
                            panic!("Shit")
                        }
                    };
                });
        });
        games.push(game);
    });
    dbg!(&games);
    // No more than 100 presses per button!
    // Some games cannot be won!
    let mut total_tokens: i64 = 0;
    for game in games {
        let mut tokens: Vec<i64> = Vec::new();
        for i in 0..100 {
            for j in 0..100 {
                let x = i * game.button_a.0 + j * game.button_b.0;
                let y = i * game.button_a.1 + j * game.button_b.1;
                if game.prize.0 == x && game.prize.1 == y {
                    tokens.push(i * 3 + j);
                }
            }
        }
        if !tokens.is_empty() {
            tokens.sort();
            total_tokens += tokens[0];
        }
    }
    total_tokens.to_string()
}

fn part_2(input: &str) -> String {
    let mut games: Vec<Game> = vec![];
    input.split("\n\n").for_each(|g| {
        let mut game = Game::default();
        g.lines().enumerate().for_each(|(i, l)| {
            l.split(":")
                .nth(1)
                .unwrap()
                .trim()
                .split(",")
                .enumerate()
                .for_each(|(j, v)| {
                    match i {
                        0 => match j {
                            0 => {
                                game.button_a.0 =
                                    v.split("+").nth(1).unwrap().parse::<i64>().unwrap();
                            }
                            1 => {
                                game.button_a.1 =
                                    v.split("+").nth(1).unwrap().parse::<i64>().unwrap();
                            }
                            _ => {
                                panic!("Shit")
                            }
                        },
                        1 => match j {
                            0 => {
                                game.button_b.0 =
                                    v.split("+").nth(1).unwrap().parse::<i64>().unwrap();
                            }
                            1 => {
                                game.button_b.1 =
                                    v.split("+").nth(1).unwrap().parse::<i64>().unwrap();
                            }
                            _ => {
                                panic!("Shit")
                            }
                        },
                        2 => match j {
                            0 => {
                                game.prize.0 = v.split("=").nth(1).unwrap().parse::<i64>().unwrap()
                                    + 10000000000000;
                            }
                            1 => {
                                game.prize.1 = v.split("=").nth(1).unwrap().parse::<i64>().unwrap()
                                    + 10000000000000;
                            }
                            _ => {
                                panic!("Shit")
                            }
                        },
                        _ => {
                            panic!("Shit")
                        }
                    };
                });
        });
        games.push(game);
    });
    dbg!(&games);
    // No more than 100 presses per button!
    // Some games cannot be won!
    let mut total_tokens: i64 = 0;
    for game in games {
        let b = (game.button_a.0 * game.prize.1 - game.button_a.1 * game.prize.0)
            / (game.button_b.1 * game.button_a.0 - game.button_a.1 * game.button_b.0);
        let a = (game.prize.0 - game.button_b.0 * b) / game.button_a.0;
        dbg!(b);
        dbg!(a);
        let eq1 = (game.button_a.0) * a + game.button_b.0 * b;
        let eq2 = (game.button_a.1) * a + game.button_b.1 * b;
        dbg!(eq1 == game.prize.0);
        dbg!(eq2 == game.prize.1);
        if eq1 == game.prize.0 && eq2 == game.prize.1 {
            total_tokens += (a * 3 + b);
        }
    }
    total_tokens.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
";

    #[test]
    fn test_1() {
        let output = part_1(TEST_INPUT);
        assert_eq!(output, "480".to_string())
    }

    #[test]
    fn test_2() {
        let output = part_2(TEST_INPUT);
        assert_eq!(output, "".to_string())
    }
}
