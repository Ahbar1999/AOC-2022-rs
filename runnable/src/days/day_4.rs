use crate::*;

pub fn solve(test_path: &str) -> std::io::Result<()> {
    let file = File::open(&test_path)?; 
    
    let file_reader = BufReader::new(file); 
    let lines = file_reader.lines(); 
    let mut count = 0;

    for line in lines {
        if let Ok(contents) = line {
            // solution logic 
            let mut pairs: Vec<Vec<u32>> = contents.split(',').map(| s | s.split('-').map(| num | num.parse::<u32>().unwrap()).collect::<Vec<u32>>()).collect();               
            // sort by start point 
            pairs.sort_by_key(| k | k[0]); 
            // println!("{:?}", &pairs); 
            // if pairs overlap at all 
            // end of first >= start of second 
            if pairs[0][1] >= pairs[1][0] {
                count += 1;
            } 
        }
    }

    println!("Answer for day_4's problem: {:?}", count); 
    
    Ok(())
}

