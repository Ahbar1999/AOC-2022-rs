use crate::*;
use std::collections::HashSet;


fn calculate_score(ch: char) -> usize {
    if ch.is_ascii_lowercase() {
        ch as usize - 'a' as usize + 1 
    } else {
        ch as usize - 'A' as usize + 27
    }
}

pub fn solve(test_path: &str) -> std::io::Result<()> {
    let file = File::open(&test_path)?; 
    
    let file_reader = BufReader::new(file); 
    let lines = file_reader.lines(); 
    let mut ans = 0;
    let mut i = 0;
    let mut rucksacks: Vec<HashSet<char>> = vec![];
    rucksacks.push(HashSet::new());
    rucksacks.push(HashSet::new());
    rucksacks.push(HashSet::new());

    for line in lines {
        if let Ok(items) = line {
            // let mut compartment_one: HashSet<char> = HashSet::new(); 
                
            
            /* PART 1 
            // let compartments = items.split_at(items.len() / 2);
            for ch in compartments.0.chars() {
                  compartment_one.insert(ch.clone());
            }
            
            for ch in compartments.1.chars() {
                if compartment_one.contains(&ch) {
                    ans += calculate_score(ch.clone());
                    // i think theres only one duplicate 
                    break;
                }
            }
            */
            for ch in items.chars() {
                rucksacks[i].insert(ch); 
            }

            if i == 2 {
                for item in rucksacks[0].intersection(&rucksacks[1]) {
                   if rucksacks[2].contains(&item) {
                        ans += calculate_score(item.clone()); 
                        break; 
                   } 
                }
                
                // reset rucksacks group
                rucksacks[0].clear();
                rucksacks[1].clear(); 
                rucksacks[2].clear();
                i = 0;
            } else { 
                i += 1;
            }
        }
    }

    println!("Answer for day_3's problem: {:?}", ans); 
    
    Ok(())
}

