use std::fs::File;
use std::io::prelude::*;

fn str_to_vec(input: &String) -> Vec<u32> {
    // Converts a single digit string to a vector of numbers
    input.chars().map(|s| s.to_digit(10).unwrap()).collect()
}

fn zip_offset_vec(items: &Vec<u32>, offset: usize) -> Vec<u32> {
    // Creates an offset vector, zips the items and returns a reduced vector of matched pairs
    let mut a_sub: Vec<u32> = Vec::new();
    a_sub.extend(&items[items.len() - offset..]);
    a_sub.extend(&items[..items.len() - offset]);
    items
        .iter()
        .zip(a_sub.iter())
        .map(|(&x, &y)| match x == y {
            true => x,
            false => 0,
        })
        .collect()
}


fn main() {
    let mut f = File::open("src/input.txt").expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    println!("With text:\n{}", contents);
    let items: Vec<u32> = str_to_vec(&contents);
    let results = zip_offset_vec(&items, 1);
    let sum: u32 = results.iter().sum();
    println!("Offset sum 1: {}", sum);
    let results = zip_offset_vec(&items, items.len() / 2);
    let sum: u32 = results.iter().sum();
    println!("Offset sum 1/2 list: {}", sum);
}
