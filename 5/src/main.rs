use std::fs::File;
use std::io::prelude::*;

fn load_file() -> Vec<isize> {
    let mut f = File::open("src/input.txt").expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    contents.lines()
            .map(|x| x.parse().unwrap())
            .collect()
}

fn execute(input: &Vec<isize>, dec_count: bool) -> u32 {
    let mut index: isize = 0;
    let mut target: isize;
    let mut count: u32 = 0;
    let mut instructions: Vec<isize> = input.clone();
    loop {
        target = index + instructions[index as usize];
        count += 1;
        if  target < 0 || target >= instructions.len() as isize {
            break
        }
        if dec_count && target - index >= 3 {
            instructions[index as usize] -= 1;
        }
        else {
            instructions[index as usize] += 1;
        }
        
        index = target;
    }
    count
}


fn main() {
    let instructions: Vec<isize> = load_file();
    println!("End count A: {}", execute(&instructions, false));
    println!("End count B: {}", execute(&instructions, true));
}
