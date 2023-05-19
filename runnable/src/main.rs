use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;
use std::cmp;
use std::env;

mod day_1 {
    use crate::*;

    pub fn solve(test_path: &str) -> std::io::Result<()> {
        let file = File::open(&test_path)?; 
        
        let file_reader = BufReader::new(file); 
        let lines = file_reader.lines(); 
        let mut max_calories = 0;
        let mut curr_calories: u32 = 0; 
        
        for line in lines {
            if let Ok(contents) = line {
                // println!("Read line {:}", contents); 
                if contents == "" {
                    max_calories = cmp::max(max_calories, curr_calories);
                    // reset calories
                    curr_calories = 0;
                } else {
                    curr_calories += str::parse::<u32>(contents.as_str().trim()).unwrap();   
                } 
            }
        }
        
        println!("Answer for day_1's problem: {:}", max_calories); 
        
        Ok(())
    }
}

fn main() {
    // call day_1 with test input
    let day = "day_1"; 
    let mut file_path = env::current_dir().unwrap();
    file_path.push("..");
    file_path.push("test_files");
    file_path.push(day);
    file_path.push("input.txt");

    // println!("{:}", file_path.to_str().unwrap()); 
    
    if let Ok(_) = day_1::solve(file_path.to_str().unwrap()) {
        println!("Successfully solved day 1's problem");
    } else {
        println!("Failed to solve day 1's problem");
    }
}
