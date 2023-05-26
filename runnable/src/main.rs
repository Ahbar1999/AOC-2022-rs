use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;
use std::env;
use crate::days::*;

#[allow(dead_code)]
mod days;

fn main() {
    // call day_x with test input
    // build the input file path 
    
    let day = "day_8";
    let mut file_path = env::current_dir().unwrap();
    file_path.push("..");
    file_path.push("test_files");
    file_path.push(day);
    file_path.push("input.txt");
    // println!("{:}", file_path.to_str().unwrap()); 
    
    println!("####################################"); 
    println!("Answer for: ");
    
    if let Ok(_) = day_8::solve(file_path.to_str().unwrap()) {
        println!("Successfully solved today's problem");
    } else {
        println!("Failed to solve day today's problem");
    }
}
