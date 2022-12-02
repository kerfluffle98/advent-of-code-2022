use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("input") {
        let mut points : i32 = 0;
        for line in lines {
            if let Ok(l) = line {
                let opponent = &l[..1];
                let me = &l[2..];

                if me == "X" {
                    points += 1;
                    match opponent {
                        "A" => points += 3,
                        "B" => points += 0,
                        "C" => points += 6,
                        _ => println!("how'd u get here")
                    }
                } else if me == "Y" {
                    points += 2;
                    match opponent {
                        "A" => points += 6,
                        "B" => points += 3,
                        "C" => points += 0,
                        _ => println!("why'd u get here")
                    }
                } else if me == "Z" {
                    points += 3;
                    match opponent {
                        "A" => points += 0,
                        "B" => points += 6, 
                        "C" => points += 3,
                        _ => println!("when'd u get here")
                    }
                }
            }
        }
        println!("{}", points);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}