use std::{fs::File, io::{BufReader, BufRead, self}};

fn main() -> io::Result<()> {
    println!("Top one");
    find_top_one()?;
    find_top_three()?;
    Ok(())
}

fn find_top_one() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut max_calories = 0;
    let mut current_value = 0;
    for line in reader.lines() {
        let line = line?;
        if line.is_empty() {
            if current_value > max_calories {
                max_calories = current_value;
            }
            current_value = 0;
            continue;
        }

        match line.parse::<i32>() {
            Ok(calories) => {
                current_value += calories
            },
            Err(e) => {
                eprintln!("Error: {}", e);
                continue;
            }
        }
    }
    
    println!("{}", max_calories);
    Ok(())
    
}

fn find_top_three() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut elves = Vec::new();
    let mut current_value = 0;

    for line in reader.lines() {
        let line = line?;
        if line.is_empty() {
            elves.push(current_value);
            current_value = 0;
            continue;
        }

        match line.parse::<i32>() {
            Ok(calories) => {
                current_value += calories
            },
            Err(e) => {
                eprintln!("Error: {}", e);
                continue;
            }
        }
    }
    
    elves.sort_by(|a, b| b.cmp(a));

    let top_three = &elves[0..3];

    let mut total = 0;
    for elf in top_three {
        println!("{}", elf);
        total += elf;
    }
    println!("Total: {}", total);
    
    Ok(())
    
}
