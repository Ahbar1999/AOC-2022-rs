use crate::*;

pub fn solve(test_path: &str) -> std::io::Result<()> {
    let file = File::open(&test_path)?; 
    
    let file_reader = BufReader::new(file); 
    let lines = file_reader.lines(); 
   
    // solution logic

    
    println!("Answer for day_2's problem: {:?}", ); 
    
    Ok(())
}
