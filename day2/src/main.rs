use std::{io, fs::File};

fn main() -> io::Result<()> {
    File::open("input.txt")?;



    Ok(())
}


enum MatchResult {
    Loss = 0,
    Draw = 3,
    Win = 6
}

enum RivalMove {
    A, //rock
    B, //paper
    C, //scissors
}