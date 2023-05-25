use crate::*;
use std::collections::HashMap;

fn find_marker_at_n(contents: &str, n: usize) {
    // solution logic         
    let mut last_n: HashMap<char, u32> = HashMap::new();
    let mut ans: usize = 0;
     
    // set start pointer to the start of the string 
    let mut start: (usize, char) = contents.chars().enumerate().next().unwrap(); 
    // println!("start: {:?}", start); 
    
    for (i, ch) in contents.chars().enumerate() {
        if !last_n.contains_key(&ch) {
            last_n.insert(ch, 0);  
        }
        
        let count = last_n.get(&ch).unwrap();
        last_n.insert(ch, count + 1);
    
        if i - start.0 + 1 > n {
            // remove contents[start]
            let count = &last_n[&start.1];
            last_n.insert(start.1, count - 1);
            if *last_n.get(&start.1).unwrap() == 0 as u32 {
                last_n.remove(&start.1); 
            }

            // increment start pointer
            start = contents.chars().enumerate().nth(start.0 + 1).unwrap();
        }

        // if count of distinct elements = 4
        if last_n.len() == n {
            ans = i + 1;
            println!("Answer for day_6's problem: {:?}", ans); 
            break; 
        }
        // println!("{:?} {:?}", start, (i, ch));
    }
}


pub fn solve(test_path: &str) -> std::io::Result<()> {
    let file = File::open(&test_path)?; 
    
    let file_reader = BufReader::new(file); 
    let lines = file_reader.lines();

    for line in lines {
        if let Ok(contents) = line { 
            // first part 
            find_marker_at_n(&contents, 4);
            println!("Second part's answer: ");
            // second part
            find_marker_at_n(&contents, 14); 
            println!("######################"); 
        }
    }
     
    Ok(())
}

