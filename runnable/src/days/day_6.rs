use crate::*;
use std::collections::HashMap;


pub fn solve(test_path: &str) -> std::io::Result<()> {
    let file = File::open(&test_path)?; 
    
    let file_reader = BufReader::new(file); 
    let lines = file_reader.lines();

    for line in lines {
        if let Ok(contents) = line { 
            
            // solution logic         
            let mut last_four: HashMap<char, u32> = HashMap::new();
            let mut ans: usize = 0;
             
            // set start pointer to the start of the string 
            let mut start: (usize, char) = contents.chars().enumerate().next().unwrap(); 
            // println!("start: {:?}", start); 
            
            for (i, ch) in contents.chars().enumerate() {
                if !last_four.contains_key(&ch) {
                    last_four.insert(ch, 0);  
                }
                
                let n = last_four.get(&ch).unwrap();
                last_four.insert(ch, n + 1);
            
                if i - start.0 + 1 > 4 {
                    // remove contents[start]
                    let count = &last_four[&start.1];
                    last_four.insert(start.1, count - 1);
                    if *last_four.get(&start.1).unwrap() == 0 as u32 {
                        last_four.remove(&start.1); 
                    }

                    // increment start pointer
                    start = contents.chars().enumerate().nth(start.0 + 1).unwrap();
                }

                // if count of distinct elements = 4
                if last_four.len() == 4 {
                    ans = i + 1;
                    println!("Answer for day_6's problem: {:?}", ans); 
                    break; 
                }

                // println!("{:?} {:?}", start, (i, ch));
            }  
        }
    }
     
    Ok(())
}

