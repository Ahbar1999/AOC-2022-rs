use crate::*;
use std::collections::HashSet;

pub fn solve(test_path: &str) -> std::io::Result<()> {
    let file = File::open(&test_path)?; 
    
    let file_reader = BufReader::new(file); 
    let lines = file_reader.lines(); 
    let mut ans = 0;

    for line in lines {
        if let Ok(items) = line {
            let mut compartment_one: HashSet<char> = HashSet::new(); 
 
            let compartments = items.split_at(items.len() / 2);
            for ch in compartments.0.chars() {
                  compartment_one.insert(ch.clone());
            }

            for ch in compartments.1.chars() {
                if compartment_one.contains(&ch) {
                    let score;
                    if ch.is_ascii_lowercase() {
                        score = ch as usize - 'a' as usize + 1; 
                    } else {
                        score = ch as usize - 'A' as usize + 27;
                    }
                    ans += score;
                    // i think theres only one duplicate 
                    break;
                }
            } 

        }
    }

    println!("Answer for day_3's problem: {:?}", ans); 
    
    Ok(())
}

