use crate::*;

// game results and moves with there respective scores
enum GameResult {
    Win = 6,
    Draw = 3,
    Loss = 0,
}

enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

struct Game {
     your_move: Move,
     opponents_move: Move,
     verdict: Option<(GameResult,u32)>,
}

impl Game {

    fn new(moves: &Vec<&str>) -> Self {
        // your_move: Move, opponents_move: Move
        let opponents_move = match moves[0] {
            "A" => {Move::Rock},
            "B" => {Move::Paper},
            _ => {Move::Scissors},
        };

        let your_move = match moves[1] {
            "X" => {Move::Rock},    
            "Y" => {Move::Paper},   
            _ => {Move::Scissors},
        };

        let mut obj = Game{your_move, opponents_move, verdict: None};
        obj.verdict = Some(obj.get_verdict());
        
        return obj;
    }

    fn get_verdict(&self) -> (GameResult, u32) {
        match (&self.your_move, &self.opponents_move) {
            (Move::Rock, Move::Paper) => {(GameResult::Loss, Move::Rock as u32 + GameResult::Loss as u32)},
            (Move::Rock, Move::Scissors) => {(GameResult::Win, 1 + 3)},
            (Move::Paper, Move::Scissors) => {(GameResult::Loss, 1 + 3)},
            (Move::Paper, Move::Rock) => {(GameResult::Draw, 1 + 3)},
            (Move::Scissors, Move::Paper) => {(GameResult::Draw, 1 + 3)},
            (Move::Scissors, Move::Rock) => {(GameResult::Draw, 1 + 3)},
            (Move::Rock, Move::Rock) => { (GameResult::Draw, 1 + 3) },
            (Move::Paper, Move::Paper) => { (GameResult::Draw, 2 + 3) },
            (Move::Scissors, Move::Scissors) => { (GameResult::Draw, 3 + 3) },
        };

        todo!("get verdict: result + score");
    }

}

pub fn solve(test_path: &str) -> std::io::Result<()> {
    let file = File::open(&test_path)?; 
    
    let file_reader = BufReader::new(file); 
    let lines = file_reader.lines(); 
    let mut running_score = 0;

    // solution logic
    for line in lines {
        // if it is a valid line
        if let Ok(contents) = line {
            // moves[0] => opponent's move, moves[1] => your move 
            let moves: Vec<&str> = contents.split_whitespace().collect();
            let game = Game::new(&moves);
            // println!("{:}", game.get_verdict());
            running_score += game.get_verdict().1;
        }

    }  
    println!("Answer for day_2's problem: {:?}", running_score); 
    
    Ok(())
}
