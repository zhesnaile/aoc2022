use std::{io, fs::File};

fn main() -> io::Result<()> {
    File::open("input.txt")?;



    Ok(())
}


enum match_result {
    Loss = 0,
    Draw = 3,
    Win = 6
}

enum rival_move {
    A, //rock
    B, //paper
    C, //scissors
}