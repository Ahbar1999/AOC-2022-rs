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
     your_move: Option<Move>,
     opponents_move: Option<Move>,
     verdict: Option<(GameResult, u32)>,
}

impl Game {

    fn new(moves: &Vec<&str>) -> Self {
        // your_move: Move, opponents_move: Move
        let opponents_move = match moves[0] {
            "A" => {Move::Rock},
            "B" => {Move::Paper},
            _ => {Move::Scissors},
        };
        
        // for second half
        let round_verdict = match moves[1] {
            "X" => {GameResult::Loss},    
            "Y" => {GameResult::Draw},   
            _ => {GameResult::Win},
        };

        /*
        // for first half
        let your_move = match moves[1] {
            "X" => {Move::Rock},    
            "Y" => {Move::Paper},   
            _ => {Move::Scissors},
        };
        */

        // let mut obj = Game{your_move, opponents_move, verdict: None}; 
        let mut obj = Game{your_move: None, opponents_move: Some(opponents_move), verdict: None};
        obj.your_move = Some(obj.get_move(&round_verdict)); 
        obj.verdict = Some(obj.get_verdict());
        
        return obj;
    }
    
    // get player 1's move given player 2's move and result for player 1 
    fn get_move(&self, result: &GameResult) -> Move {
        match (self.opponents_move.as_ref().unwrap(), result) {
            (Move::Rock, GameResult::Win) => Move::Paper,
            (Move::Rock, GameResult::Loss) => Move::Scissors,
            (Move::Paper, GameResult::Win) => Move::Scissors,
            (Move::Paper, GameResult::Loss) => Move::Rock,
            (Move::Scissors, GameResult::Win) => Move::Rock,
            (Move::Scissors, GameResult::Loss) => Move::Paper,
            (Move::Rock, GameResult::Draw) => Move::Rock,
            (Move::Paper, GameResult::Draw) => Move::Paper,
            (Move::Scissors, GameResult::Draw) => Move::Scissors,
        }
    }

    fn get_verdict(&self) -> (GameResult, u32) { 
        match (self.your_move.as_ref().unwrap(), self.opponents_move.as_ref().unwrap()) {
            (Move::Rock, Move::Paper) => (GameResult::Loss, Move::Rock as u32 + GameResult::Loss as u32),
            (Move::Rock, Move::Scissors) => (GameResult::Win, Move::Rock as u32 +  GameResult::Win as u32),
            (Move::Paper, Move::Scissors) => (GameResult::Loss, Move::Paper as u32 +  GameResult::Loss as u32),
            (Move::Paper, Move::Rock) => (GameResult::Win, GameResult::Win as u32 + Move::Paper as u32),
            (Move::Scissors, Move::Paper) => (GameResult::Win, GameResult::Win as u32 + Move::Scissors as u32),
            (Move::Scissors, Move::Rock) => (GameResult::Loss, GameResult::Loss as u32 + Move::Scissors as u32),
            (Move::Rock, Move::Rock) => (GameResult::Draw, GameResult::Draw as u32 + Move::Rock as u32),
            (Move::Paper, Move::Paper) => (GameResult::Draw, GameResult::Draw as u32 + Move::Paper as u32),
            (Move::Scissors, Move::Scissors) => (GameResult::Draw, GameResult::Draw as u32 + Move::Scissors as u32),
        }
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
            running_score += game.verdict.unwrap().1;
        }

    }  
    
    println!("Answer for day_2's problem: {:?}", running_score); 
    
    Ok(())
}
