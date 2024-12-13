fn main() {
    let input = include_str!("./input_1.txt");
    let output_1 = part_1(input);
    println!("{}", output_1);
    let output_2 = part_2(input);
    println!("{}", output_2);
}

#[derive(Default, Debug)]
struct Game {
    button_a: (u64, u64),
    button_b: (u64, u64),
    prize: (u64, u64),
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
                                    v.split("+").nth(1).unwrap().parse::<u64>().unwrap();
                            }
                            1 => {
                                game.button_a.1 =
                                    v.split("+").nth(1).unwrap().parse::<u64>().unwrap();
                            }
                            _ => {
                                panic!("Shit")
                            }
                        },
                        1 => match j {
                            0 => {
                                game.button_b.0 =
                                    v.split("+").nth(1).unwrap().parse::<u64>().unwrap();
                            }
                            1 => {
                                game.button_b.1 =
                                    v.split("+").nth(1).unwrap().parse::<u64>().unwrap();
                            }
                            _ => {
                                panic!("Shit")
                            }
                        },
                        2 => match j {
                            0 => {
                                game.prize.0 = v.split("=").nth(1).unwrap().parse::<u64>().unwrap();
                            }
                            1 => {
                                game.prize.1 = v.split("=").nth(1).unwrap().parse::<u64>().unwrap();
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
    "".to_string()
}

fn part_2(input: &str) -> String {
    "".to_string()
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
        assert_eq!(output, "".to_string())
    }

    #[test]
    fn test_2() {
        let output = part_2(TEST_INPUT);
        assert_eq!(output, "".to_string())
    }
}
