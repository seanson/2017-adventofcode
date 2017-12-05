use std::fs::File;
use std::io::prelude::*;

fn parse_line(line: &str) -> Vec<u32> {
    line.split_whitespace().map(|s| s.parse().unwrap()).collect()
}

fn parse_file(file: &str) -> Vec<Vec<u32>> {
    file.lines().map(|line| parse_line(line)).collect()
}

fn vec_to_checksum(list: &Vec<u32>) -> u32 {
    let mut new_list: Vec<u32> = list[..].to_vec();
    new_list.sort();
    new_list[new_list.len()-1] - new_list[0]
}


fn vec_to_div(list: &Vec<u32>) -> u32 {
    list.iter()
        .map(|&x| list.iter()
                      .map(|&y| match x % y == 0 && x != y {
                                    true => x / y,
                                    false => 0
                                    }
                          )
                      .sum::<u32>()
        ).sum()
}


fn main() {
    let mut f = File::open("src/input.txt").expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something went wrong reading the file");
    println!("With text:\n{}", contents);
    let items: Vec<Vec<u32>> = parse_file(&contents);
    let result_1: u32 = items.iter()
                              .map(|row| vec_to_checksum(&row))
                              .sum();
    println!("Checksum 1: {}", result_1);
    let result_2: u32 = items.iter()
                             .map(|row| vec_to_div(&row))
                             .sum();
    println!("Checksum 2: {}", result_2);
}