use std::collections::HashMap;

fn main() {
    let input = include_str!("./input_1.txt");
    let output_1 = part_1(input, 101, 103, 100);
    println!("{}", output_1);
    let output_2 = part_2(input, 101, 103, 10000);
    println!("{}", output_2);
}

#[derive(Default, Debug, Clone, Copy)]
struct Robot {
    position: (i32, i32),
    velocity: (i32, i32),
}

fn part_1(input: &str, width: i32, height: i32, time: i32) -> String {
    let mut robots: Vec<Robot> = vec![];
    input.lines().for_each(|l| {
        let mut robot = Robot::default();
        let mut parts = l.split_whitespace();
        let mut p = parts.next().unwrap().split("=").last().unwrap().split(",");
        robot.position.0 = p.next().unwrap().parse().unwrap();
        robot.position.1 = p.next().unwrap().parse().unwrap();
        let mut v = parts.next().unwrap().split("=").last().unwrap().split(",");
        robot.velocity.0 = v.next().unwrap().parse().unwrap();
        robot.velocity.1 = v.next().unwrap().parse().unwrap();
        robots.push(robot);
    });
    for _ in 0..time {
        robots.iter_mut().for_each(|robot| {
            robot.position.0 += robot.velocity.0;
            robot.position.1 += robot.velocity.1;
            if robot.position.0 < 0 {
                robot.position.0 += width;
            }
            if robot.position.0 >= width {
                robot.position.0 -= width;
            }
            if robot.position.1 < 0 {
                robot.position.1 += height;
            }
            if robot.position.1 >= height {
                robot.position.1 -= height;
            }
        });
    }

    let width_halfway = (width - 1) / 2;
    let height_halfway = (height - 1) / 2;
    let top_left = ((-1, width_halfway), (-1, height_halfway));
    let mut top_left_count = 0;
    let top_right = ((width_halfway, width), (-1, height_halfway));
    let mut top_right_count = 0;
    let bottom_left = ((-1, width_halfway), (height_halfway, height));
    let mut bottom_left_count = 0;
    let bottom_right = ((width_halfway, width), (height_halfway, height));
    let mut bottom_right_count = 0;

    for robot in robots {
        // top left
        if robot.position.0 > top_left.0 .0
            && robot.position.0 < top_left.0 .1
            && robot.position.1 > top_left.1 .0
            && robot.position.1 < top_left.1 .1
        {
            top_left_count += 1;
        }
        if robot.position.0 > top_right.0 .0
            && robot.position.0 < top_right.0 .1
            && robot.position.1 > top_right.1 .0
            && robot.position.1 < top_right.1 .1
        {
            top_right_count += 1;
        }
        if robot.position.0 > bottom_left.0 .0
            && robot.position.0 < bottom_left.0 .1
            && robot.position.1 > bottom_left.1 .0
            && robot.position.1 < bottom_left.1 .1
        {
            bottom_left_count += 1;
        }
        if robot.position.0 > bottom_right.0 .0
            && robot.position.0 < bottom_right.0 .1
            && robot.position.1 > bottom_right.1 .0
            && robot.position.1 < bottom_right.1 .1
        {
            bottom_right_count += 1;
        }
    }
    let output = top_left_count * top_right_count * bottom_left_count * bottom_right_count;
    output.to_string()
}

fn part_2(input: &str, width: i32, height: i32, time: i32) -> String {
    let mut robots: Vec<Robot> = vec![];
    input.lines().for_each(|l| {
        let mut robot = Robot::default();
        let mut parts = l.split_whitespace();
        let mut p = parts.next().unwrap().split("=").last().unwrap().split(",");
        robot.position.0 = p.next().unwrap().parse().unwrap();
        robot.position.1 = p.next().unwrap().parse().unwrap();
        let mut v = parts.next().unwrap().split("=").last().unwrap().split(",");
        robot.velocity.0 = v.next().unwrap().parse().unwrap();
        robot.velocity.1 = v.next().unwrap().parse().unwrap();
        robots.push(robot);
    });
    let mut average_distances: Vec<(usize, i32)> = vec![];
    for t in 1..=time {
        robots.iter_mut().for_each(|robot| {
            robot.position.0 += robot.velocity.0;
            robot.position.1 += robot.velocity.1;
            if robot.position.0 < 0 {
                robot.position.0 += width;
            }
            if robot.position.0 >= width {
                robot.position.0 -= width;
            }
            if robot.position.1 < 0 {
                robot.position.1 += height;
            }
            if robot.position.1 >= height {
                robot.position.1 -= height;
            }
        });
        let map_string = (".".repeat(width as usize) + "\n").repeat(height as usize);
        // println!("{:?}", map_string);
        let mut map: HashMap<(i32, i32), char> = HashMap::new();
        map_string.lines().enumerate().for_each(|(j, l)| {
            l.chars().enumerate().for_each(|(i, c)| {
                map.insert((i as i32, j as i32), c);
            })
        });
        let mut delta_x_sum = 0;
        let mut delta_y_sum = 0;
        for r1 in robots.iter() {
            for r2 in robots.iter() {
                delta_x_sum += (r1.position.0 - r2.position.0).pow(2);
                delta_y_sum += (r1.position.1 - r2.position.1).pow(2);
            }
        }
        average_distances.push((t as usize, (delta_x_sum + delta_y_sum)));
    }
    average_distances.sort_by(|a, b| a.1.cmp(&b.1));
    dbg!(&average_distances[0..5]);
    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
";

    #[test]
    fn test_1() {
        let output = part_1(TEST_INPUT, 11, 7, 100);
        assert_eq!(output, "12".to_string())
    }

    #[test]
    fn test_2() {
        let output = part_2(TEST_INPUT, 11, 7, 100);
        assert_eq!(output, "".to_string())
    }
}
