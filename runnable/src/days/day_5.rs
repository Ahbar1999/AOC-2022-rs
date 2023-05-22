use crate::*;

pub fn solve(test_path: &str) -> std::io::Result<()> {
    let file = File::open(&test_path)?; 
    
    let file_reader = BufReader::new(file); 
    let lines = file_reader.lines(); 
    let mut crate_arrangement: Vec<String> = vec![]; 
    let mut building_stack = true;

    let mut crate_stacks: Vec<Vec<char>> = vec![];
    for line in lines {
        if let Ok(contents) = line {
            if building_stack {
                if contents == "" {
                    building_stack = false; 
                } else {
                    crate_arrangement.push(contents);
                }
            } else {
                // build stack the first time
                if crate_stacks.len() == 0 {
                    // println!("{:?}", crate_arrangement);
                    // number of stacks 
                    let n = crate_arrangement.pop().unwrap().chars().filter(| ch | !ch.is_whitespace()).collect::<Vec<char>>().len(); 
                    println!("{:}", n);

                    crate_arrangement.reverse();
                    // println!("{:?}", crate_arrangement);
                    crate_stacks = vec![Vec::new(); n];
                    // add items to stacks
                    for arr in crate_arrangement.iter() {
                        for idx in 0..n {
                            let item = arr.chars().nth(4 * idx + 1).unwrap(); 
                            if item != ' ' {
                                crate_stacks[idx].push(item);
                            }
                        }     
                    }
                    println!("{:?}", crate_stacks);
                } 
                // process move statements
                let arg_string = contents.chars()
                    .filter(| ch | !ch.is_ascii_alphabetic())
                    .collect::<String>();
                let args = arg_string.trim()
                    .split(" ")
                    .filter(| s | s != &"")
                    .map(| s | s.parse().unwrap())
                    .collect::<Vec<u32>>();
                // args.trim().split(" ").map(| s | s.parse().unwrap()).collect();
                // println!("args: {:?}", args);
                
                for _ in 0..args[0] {
                    let item = crate_stacks[args[1] as usize - 1].pop().unwrap();
                    crate_stacks[args[2] as usize - 1].push(item);             
                }
                
                // println!("curr state of stacks: {:?}", crate_stacks);
            } 
        }
    }

    let ans = crate_stacks.iter().map(| item | item.last().unwrap().to_string()).reduce(| acc, ch | acc + &ch).unwrap();  
        
    println!("Answer for day_5's problem: {:?}", ans); 
 
    Ok(())
}

