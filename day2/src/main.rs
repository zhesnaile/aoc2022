use std::{io::{self, BufReader, BufRead}, fs::File};

#[derive(Copy, Clone)]
enum RPSMatchResult {
    Loss = 0,
    Draw = 3,
    Win = 6
}

#[derive(Copy, Clone)]
enum RPSMove {
    Rock = 1,
    Paper,
    Scissors,
}

fn find_move(a: &str) -> RPSMove {
    match a {
        "A"|"X" => RPSMove::Rock,
        "B"|"Y" => RPSMove::Paper,
        "C"|"Z" => RPSMove::Scissors,
        _ => panic!("Invalid value read")
    }
}
/**
 * 
 */
fn play_round(rival_move :RPSMove, player_move: RPSMove) -> RPSMatchResult {
    let round = (rival_move, player_move);
    
    match round {
        (RPSMove::Rock, RPSMove::Paper)| (RPSMove::Paper, RPSMove::Scissors) | (RPSMove::Scissors, RPSMove::Rock) => RPSMatchResult::Win,
        (RPSMove::Paper, RPSMove::Rock) | (RPSMove::Scissors, RPSMove::Paper) | (RPSMove::Rock, RPSMove::Scissors) => RPSMatchResult::Loss,
        (_, _) => RPSMatchResult::Draw,
    }

}

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut player_points = 0;

    for line in reader.lines() {
        let line = line?;
        let rps_match: Vec<&str> = line.split(' ').collect();
        
        let rival_move = find_move(rps_match[0]);
        let player_move = find_move(rps_match[1]);
        
        let round_points= play_round(rival_move, player_move) as isize + player_move as isize; 

        player_points += round_points;
    }

    println!("Total Points: {}", player_points);
    Ok(())
}