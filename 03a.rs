use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("input") {
        let mut priority_sum : u32 = 0;
        for line in lines {
            if let Ok(l) = line {
                let (compartment1, compartment2) = l.split_at(l.chars().count()/2);
                for c in compartment1.chars() {
                    if compartment2.contains(c) {
                        priority_sum += get_priority(c);
                        break;
                    }
                }
            }
        }
        println!("{}", priority_sum);
    }
}

// Wowweee. Rust doesn't have anything like this built in.
fn get_priority(c : char) -> u32 {
    let priority = " abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut counter = 0;
    for x in priority.chars() {
        if x == c {
            return counter;
        }
        counter += 1;
    }
    return counter;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}