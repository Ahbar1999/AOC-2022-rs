use crate::*;
use std::collections::HashSet;

/*
#[derive(Debug, Clone)]
struct TallestTree {
    height: u32,
    coordinates: (usize, usize), 
}
*/

pub fn solve(test_path: &str) -> std::io::Result<()> {
    let file = File::open(&test_path)?; 
    
    let file_reader = BufReader::new(file); 
    let lines = file_reader.lines(); 
    let mut visible_trees: HashSet<(usize, usize)> = HashSet::new();
    let mut tree_grid: Vec<Vec<u32>> = vec![];

    for line in lines {
        if let Ok(contents) = line {
            tree_grid.push(contents.chars().map(| ch | ch.to_digit(10).unwrap()).collect()); 
        }
    }
    // for visualizing the trees 
    // let mut tree_map: Vec<Vec<char>> = vec![vec!['.'; tree_grid[0].len()]; tree_grid.len()];
    
    // use monotonic stack
    
    // for every row sweep left once and sweep right once
    // for every col sweep right once and sweep left once
    for (r, row) in tree_grid.iter().enumerate() {
        let mut stack: Vec<u32> = vec![]; 
        // sweep left to right for view on each tree from left 
        for (c, tree) in row.iter().enumerate() { 
            if !stack.is_empty() && stack.last().unwrap() >= tree {
               continue; 
            } 

            stack.push(*tree);
            visible_trees.insert((r, c));
        }
        // currently the stack has the trees that can be seen from the left at row: r 
        // println!("View from left for row: {:?}: {:?}", r, stack); 
        
        stack.clear();
        // now sweep right to left for view on each tree from left 
        for c in (0..row.len()).rev() { 
            
            let tree = tree_grid[r][c];
            if !stack.is_empty() && stack.last().unwrap() >= &tree {
               continue; 
            } 

            stack.push(tree);    
            visible_trees.insert((r, c));
        }

        // println!("View from right for row: {:?}: {:?}", r, stack); 
    }

    for c in 0..tree_grid[0].len() {
        let mut stack: Vec<u32> = vec![]; 
        // sweep left to right for view on each tree from left 
        for r in 0..tree_grid.len() { 
            let tree = tree_grid[r][c];
            if !stack.is_empty() && stack.last().unwrap() >= &tree {
               continue; 
            } 

            stack.push(tree);
            visible_trees.insert((r, c));
        }

        // currently the stack has the trees that can be seen from the top at col: c  
        // println!("View from top for column : {:?}: {:?}", c, stack); 
        
        stack.clear();
        // now sweep right to left for view on each tree from left 
        for r in (0..tree_grid.len()).rev() { 
            let tree = tree_grid[r][c];
            if !stack.is_empty() && stack.last().unwrap() >= &tree {
               continue; 
            } 

            stack.push(tree);
            visible_trees.insert((r, c));
        }
        // println!("View from bottom for column : {:?}: {:?}", c, stack); 
    }
    
    /*
    for (row, col) in visible_trees.iter() {
        tree_map[*row][*col] = '#';
    }
    for (r, row) in tree_map.iter().enumerate() {
        println!("{:?}:    {:?}", r, row); 
    }
    */
    
    println!("day_8's problem: {:?}", visible_trees.len()); 
    
    Ok(())
}

