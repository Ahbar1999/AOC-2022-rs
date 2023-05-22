use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;

// use std::cmp;
use std::env;
use crate::days::day_5;

mod days;

fn main() {
    // call day_x with test input
    // build the input file path 
    
    let day = "day_5";
    let mut file_path = env::current_dir().unwrap();
    file_path.push("..");
    file_path.push("test_files");
    file_path.push(day);
    file_path.push("input.txt");
    // println!("{:}", file_path.to_str().unwrap()); 
    
    if let Ok(_) = day_5::solve(file_path.to_str().unwrap()) {
        println!("Successfully solved day 5's problem");
    } else {
        println!("Failed to solve day 5's problem");
    }
}
