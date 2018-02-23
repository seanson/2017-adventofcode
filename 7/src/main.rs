extern crate regex;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use regex::Regex;

// fn load_file() -> Vec<HashMap<&str, &str>> {
//     let re = Regex::new(r"(?<name>\w+) \((?<weight>\d+)\)(?: -> )?(?<hold>(\w+,? ?)+)?").unwrap();
//     let mut f = File::open("src/input.txt").expect("file not found");
//     let mut contents = String::new();
//     f.read_to_string(&mut contents)
//         .expect("something went wrong reading the file");        
//     contents.lines()
//             .flat_map(|line| {
//                 let captures = re.captures(*line);
//                 re.capture_names().map(|name| (name, captures.get(name)))
//             })
//             .collect()
// }


fn main() {
    // let items = load_file();
    let re = Regex::new(r"(?P<name>\w+) \((?P<weight>\d+)\)(?: -> )?(?P<hold>(\w+,? ?)+)?").unwrap();
    let mut f = File::open("src/input.txt").expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");        
    let results: Vec<HashMap<&str, &str>> = contents.lines()
            .map(|line| {
                let captures = re.captures(line).unwrap();
                re.capture_names().map(|name| (name.unwrap(), captures.name(name.unwrap())
                                                                      .unwrap()
                                                                      .as_str()))
                                  .collect()
            })
            .collect();
}
