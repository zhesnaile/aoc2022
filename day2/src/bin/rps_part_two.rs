use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

#[derive(Copy, Clone)]
enum RPSMatchResult {
    Loss = 0,
    Draw = 3,
    Win = 6,
}

#[derive(Copy, Clone)]
enum RPSMove {
    Rock = 1,
    Paper,
    Scissors,
}

impl RPSMove {
    fn new(a: &str) -> RPSMove {
        match a {
            "A" => RPSMove::Rock,
            "B" => RPSMove::Paper,
            "C" => RPSMove::Scissors,
            _ => panic!("Invalid value read")
        }
    }
}

impl RPSMatchResult {
    fn new(a: &str) -> RPSMatchResult {
        match a {
            "X" => Self::Loss,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => panic!("Invalid value read")
        }
    }
}

fn guess_move(rival_move: RPSMove, match_result: RPSMatchResult) -> RPSMove {
    use RPSMatchResult::{Draw, Loss, Win};
    use RPSMove::{Rock, Paper, Scissors};

    let round = (rival_move, match_result);

    match round {
        (Rock, Loss)
        | (Paper, Win)
        | (Scissors, Draw) => Scissors,
        (Paper, Loss)
        | (Scissors, Win)
        | (Rock, Draw) => Rock,
        (_, _) => Paper,
    }
}

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut player_points = 0;

    for line in reader.lines() {
        let line = line?;
        let rps_match: Vec<&str> = line.split(' ').collect();

        let rival_move = RPSMove::new(rps_match[0]);
        let match_result = RPSMatchResult::new(rps_match[1]);

        let round_points = guess_move(rival_move, match_result) as isize + match_result as isize;

        player_points += round_points;
    }

    println!("Total Points: {}", player_points);
    Ok(())
}
