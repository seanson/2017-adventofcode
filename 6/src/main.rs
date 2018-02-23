#![feature(iter_rfold)]

use std::fs::File;
use std::io::prelude::*;


fn load_file() -> Vec<u32> {
    let mut f = File::open("src/input.txt").expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    contents.split_whitespace().map(|i| i.parse().unwrap())
            .collect::<Vec<u32>>()
}

fn get_max(input: Vec<u32>) -> (usize, u32) {
    input.iter().enumerate() // inspired by the amazing anisoptera
        .fold((0, 0), |acc, (index, &x)| match x > acc.1 {
            true => (index, x),
            false => acc,
        })
}

fn main() {
    let bins: Vec<u32> = load_file();
    println!("Initial list: {:?}", bins);
    println!("get_max: {:?}", get_max(bins));
}
