use crate::*;
// use std::collections::HashMap;
// use std::rc::Rc;

/*
struct Dir {
    // name of this directory
    name: String,
    // files that this directory contains with file sizes
    files: HashMap<String, u32>,
    // parent directory 
    parent: Option<Rc<Dir>>,
    // directories that this directory contains
    children: Vec<Rc<Dir>> 
}
*/

#[derive(Debug)]
struct Dir {
    name: String,
    size: u32,
}

pub fn solve(test_path: &str) -> std::io::Result<()> {
    let file = File::open(&test_path)?; 
    
    let file_reader = BufReader::new(file); 
    let lines = file_reader.lines();
    // stack 
    let mut stack: Vec<Dir> = vec![]; 
    let mut dirs: Vec<Dir> = Vec::new();
    /*
    let root: Rc<Dir> = Rc::new(Dir{name: "/".to_string(), files: HashMap::new(), parent: None, children: vec![]});
    let mut curr: Rc<Dir> = root.clone();

    for line in lines {
        if let Ok(contents) = line { 
            if contents.starts_with("$") {
                // process commands
                let (cmd, arg) = contents.split_once(" ").unwrap();
                match cmd {
                    "cd" => { 
                        match arg {
                            "/" => { curr = root.clone(); },
                            ".." => { curr = curr.parent.clone().unwrap(); },
                            _ => {
                                let new_dir = Rc::new(Dir{name: arg.clone().to_string(), files: HashMap::new(), parent: Some(curr.clone()), children: vec![] });
                                curr.children.push(new_dir.clone()); 
                            } 
                        }
                    },
                    // since there are only two commands cmd and ls, so this has to be ls
                    _ => {},
                }
            } else {
                // process printed info
           
            }
        }
    }
    */
    for line in lines {
        if let Ok(contents) = line {
            if contents.starts_with("$") {
                // process commands
                let cmdline = contents.split_at(2).1;
                // println!("{:?}", cmdline); 
                match cmdline.starts_with("cd") {
                    true => {
                        // get the args
                        let (_, arg) = cmdline.split_once(" ").unwrap();
                        match arg {
                            // move out
                            ".." => { 
                                let popped = stack.pop().unwrap();
                                let mut top = stack.last_mut().unwrap(); 
                                top.size += popped.size;
                                dirs.push(popped);
                            },
                            // move in
                            _ => { 
                                stack.push(Dir{name: arg.to_string(), size: 0}); 
                            } 
                        }
                    },
                    // ls
                    false => { /* do nothing, ignore the command */ },
                } 
            } else {
                // process printed info
                if contents.starts_with("dir") {
                    continue;
                } 
                // file description
                let mut top = stack.last_mut().unwrap();
                // get the file size
                let (filesize, filename) = contents.split_once(" ").unwrap();  
                top.size += filesize.parse::<u32>().unwrap();
            }
        }
        // println!("{:?}", stack); 
    }

    while !stack.is_empty() {
        let popped = stack.pop().unwrap();
        if !stack.is_empty() {
            let mut top = stack.last_mut().unwrap(); 
            top.size += popped.size;
        }
        dirs.push(popped);
    }

    println!("day 7 Part 1:{:?}", dirs.iter().filter(| d | d.size < 100000).map(| d | d.size).sum::<u32>()); 

    Ok(())
}

