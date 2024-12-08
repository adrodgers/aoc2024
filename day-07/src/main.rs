use std::collections::HashSet;

use itertools::Itertools;

fn main() {
    let input = include_str!("./input_1.txt");
    let output_1 = part_1(input);
    println!("{}", output_1);
    let output_2 = part_2(input);
    println!("{}", output_2);
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
enum Operation {
    Add,
    Multiply,
    Concat,
}

#[derive(Debug)]
struct Equation {
    answer: u64,
    inputs: Vec<u64>,
}

impl Equation {
    fn check_possible(&self, operators: Vec<Operation>) -> bool {
        let mut ans: u64 = self.inputs[0];
        for (i, op) in operators.into_iter().enumerate() {
            match op {
                Operation::Add => ans += self.inputs[i + 1],
                Operation::Multiply => ans *= self.inputs[i + 1],
                Operation::Concat => {
                    ans = (ans.to_string() + &self.inputs[i + 1].to_string())
                        .parse()
                        .unwrap()
                }
            };
            if ans > self.answer {
                return false;
            }
        }
        ans == self.answer
    }
}

fn part_1(input: &str) -> String {
    let mut equations: Vec<Equation> = vec![];
    input.lines().for_each(|l| {
        let mut parts = l.split(':');
        // println!("{:?}", parts);
        let answer = parts.next().unwrap().parse().unwrap();
        let inputs = parts
            .next()
            .unwrap()
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        equations.push(Equation { answer, inputs })
    });
    // println!("{:?}", equations);
    let mut output = 0;
    // generate all possible operation combinations...????
    for (i, eq) in equations.iter().enumerate() {
        println!("{:?} of {:?}", i, equations.len() - 1);
        let mut op_combinations_map: HashSet<Vec<Operation>> = HashSet::new();
        // println!("{:?}", eq.inputs.len());
        // [Operation::Add, Operation::Multiply]
        //     .iter()
        //     .combinations_with_replacement(eq.inputs.len() - 1)
        //     .for_each(|c| {
        //         // op_combinations_map.insert(c);
        //         c.clone().into_iter().permutations(c.len()).for_each(|o| {
        //             op_combinations_map.insert(o);
        //         });
        //     });
        let equations = (0..eq.inputs.len() - 1)
            .map(|_| [Operation::Add, Operation::Multiply])
            .multi_cartesian_product()
            .for_each(|eq| {
                op_combinations_map.insert(eq);
            });

        // repeat_n([Operation::Add, Operation::Multiply],eq.inputs.len() - 1 )
        println!("map len = {:?}", op_combinations_map.len());
        for (i, op) in op_combinations_map.into_iter().enumerate() {
            // println!("op iteration: {:?}", i);
            if eq.check_possible(op.clone()) {
                // println!("VALID");
                // println!("{:?}", op);
                // println!("{:?}", eq.answer);
                // println!("{:?}", eq.inputs);
                output += eq.answer;
                break;
            };
        }
    }

    output.to_string()
}

fn part_2(input: &str) -> String {
    let mut equations: Vec<Equation> = vec![];
    input.lines().for_each(|l| {
        let mut parts = l.split(':');
        // println!("{:?}", parts);
        let answer = parts.next().unwrap().parse().unwrap();
        let inputs = parts
            .next()
            .unwrap()
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        equations.push(Equation { answer, inputs })
    });
    // println!("{:?}", equations);
    let mut output = 0;
    // generate all possible operation combinations...????
    for (i, eq) in equations.iter().enumerate() {
        println!("{:?} of {:?}", i, equations.len() - 1);
        let mut op_combinations_map: HashSet<Vec<Operation>> = HashSet::new();
        let equations = (0..eq.inputs.len() - 1)
            .map(|_| [Operation::Add, Operation::Multiply, Operation::Concat])
            .multi_cartesian_product()
            .for_each(|eq| {
                op_combinations_map.insert(eq);
            });

        // repeat_n([Operation::Add, Operation::Multiply],eq.inputs.len() - 1 )
        println!("map len = {:?}", op_combinations_map.len());
        for (i, op) in op_combinations_map.into_iter().enumerate() {
            // println!("op iteration: {:?}", i);
            if eq.check_possible(op.clone()) {
                // println!("VALID");
                // println!("{:?}", op);
                // println!("{:?}", eq.answer);
                // println!("{:?}", eq.inputs);
                output += eq.answer;
                break;
            };
        }
    }

    output.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";

    #[test]
    fn test_1() {
        let output = part_1(TEST_INPUT);
        assert_eq!(output, "3749".to_string())
    }
    #[test]
    fn test_2() {
        let output = part_2(TEST_INPUT);
        assert_eq!(output, "11387".to_string())
    }
}
