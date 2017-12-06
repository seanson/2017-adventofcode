use std::fs::File;
use std::io::prelude::*;

fn split_line(input: &str) -> Vec<String> {
    input.split_whitespace().map(|x| x.to_string()).collect()
}


fn sort_string(input: String) -> String {
    let mut anagram = input.into_bytes();
    anagram.sort();
    String::from_utf8(anagram).unwrap()
}


fn check_unique(input: Vec<String>) -> u16 {
    let mut check: Vec<String> = input.clone();
    check.sort();
    check.dedup();
    match input.len() == check.len() {
        true => 1,
        false => 0,
    }
}

fn check_unique_anagram(input: Vec<String>) -> u16 {
    let sorted_strings = input
        .iter()
        .map(|word| sort_string(word.to_string()))
        .collect();
    check_unique(sorted_strings)
}

fn load_file() -> String {
    let mut f = File::open("src/input.txt").expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    contents
}


fn parse_file_unique() -> u16 {
    load_file()
        .lines()
        .map(|line| check_unique(split_line(line)))
        .sum()
}

fn parse_file_anagram_unique() -> u16 {
    load_file()
        .lines()
        .map(|line| check_unique_anagram(split_line(line)))
        .sum()
}

fn main() {
    let unique: u16 = parse_file_unique();
    println!("With combinations: {}", unique);

    let unique: u16 = parse_file_anagram_unique();
    println!("With anagram combinations: {}", unique);
}
