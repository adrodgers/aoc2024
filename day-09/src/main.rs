fn main() {
    let input = include_str!("./input_1.txt");
    let output_1 = part_1(input);
    println!("{}", output_1);
    // let output_2 = part_2(input);
    // println!("{}", output_2);
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
    // println!("files: {:?}", file_ids.len());
    // println!("space: {:?}", free_space.len());
    // // println!("files: {:?}", file_ids);
    // // println!("space: {:?}", free_space);
    // println!("last file index: {:?}", last_file_index);
    // while !free_space.is_empty() {
    //     // println!("{:?}", last_file_index);
    //     // take next available space, add last file to front of file vecdeq
    //     if let Some(space) = free_space.pop_front() {
    //         if space.index < last_file_index {
    //             if let Some(file) = file_ids.pop_back() {
    //                 // println!("moving space {:?}, file {:?}", space, file);
    //                 file_ids.push_front(File {
    //                     id: file.id,
    //                     index: space.index,
    //                 });
    //                 last_file_index -= 1;
    //             }
    //         } else {
    //             // reached space that is after the last file index, break
    //             println!(
    //                 "Exit with next space: {:?} and last_file_index {:?}",
    //                 space, last_file_index
    //             );
    //             free_space.clear();
    //         }
    //     } else {
    //         break;
    //     }
    // }
    // // println!("files: {:?}", file_ids);
    // // println!("space: {:?}", free_space);
    // let mut checksum: u128 = 0;
    // println!("{:?}", file_ids.len());
    // for file in file_ids {
    //     checksum += file.id * file.index;
    // }
    // checksum.to_string()
}

fn part_2(input: &str) -> String {
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
