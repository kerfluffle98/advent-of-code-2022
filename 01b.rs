use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("input") {
        let mut highest_sums = vec![0,0,0];
        let mut temp_sum : i32 = 0;

        for line in lines {
            if let Ok(l) = line {
                if l == "" {
                    if temp_sum > highest_sums[2] {
                        highest_sums.push(temp_sum);
                        highest_sums.sort();
                        highest_sums.reverse();
                        while highest_sums.len() > 3 {
                            highest_sums.pop();
                        }
                    }
                    temp_sum = 0;
                } else {
                    temp_sum += l.parse::<i32>().unwrap();
                }
            }
        }

        for n in &highest_sums {
            println!("{}",n);
        }
        
        let sum : i32 = highest_sums.iter().sum();
        println!("Sum: {}", sum);
    }
} 

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}