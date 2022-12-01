use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("input") {
        let mut highest_sum : i32 = 0;
        let mut temp_sum : i32 = 0;
        for line in lines {
            if let Ok(l) = line {
                if l != "" {
                    temp_sum += l.parse::<i32>().unwrap();
                } else {
                    if temp_sum > highest_sum {
                        highest_sum = temp_sum;
                    }
                    temp_sum = 0;
                }
            }
        }
        println!("{}",highest_sum);
    }
} 

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}