use crate::*;

fn update_max_calories(max_calories: &mut [u32], curr_calories: u32) {
    for i in 0..max_calories.len() {
        // update max calories at each index
        if max_calories[i] <= curr_calories {
            if i + 1 <= 2 {     
                if i + 2 <= 2 {
                    max_calories[i + 2] = max_calories[i + 1];
                } 
                max_calories[i + 1] = max_calories[i];
            }
            max_calories[i] = curr_calories;
            break; 
        }
    } 
}

pub fn solve(test_path: &str) -> std::io::Result<()> {
    let file = File::open(&test_path)?; 
    
    let file_reader = BufReader::new(file); 
    let lines = file_reader.lines(); 
    let mut max_calories = [0; 3];
    let mut curr_calories: u32 = 0; 
    
    for line in lines {
        if let Ok(contents) = line {
            // println!("Read line {:}", contents); 
            if contents == "" {
                update_max_calories(&mut max_calories, curr_calories); 
                // println!("{:?}", max_calories);
                
                // reset counter of calories
                curr_calories = 0;
            } else {
                curr_calories += str::parse::<u32>(contents.as_str().trim()).unwrap();   
            } 
        }
    }
    // finish up remaining curr_calories 
    update_max_calories(&mut max_calories, curr_calories); 
    // println!("{:?}", max_calories);

    println!("Answer for day_1's problem: {:?}", max_calories.into_iter().sum::<u32>()); 
    
    Ok(())
}

