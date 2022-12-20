use std::{fs::File, io::{self, BufRead, BufReader}};

type Compartment<'a> = &'a str;

type Rucksack<'a> = (Compartment<'a>, Compartment<'a>);

fn calculate_priority(c: char) -> u32 {
    match c {
        'a'..='z' => c as u32 - 'a' as u32 + 1,
        'A'..='Z' => c as u32 - 'A' as u32 + 27,
        _ => panic!("wtf not a valid letter")
    }
}


fn get_common_value(rucksack: Rucksack) -> u32 {
    let mut c = '0';
    'outer: for compartment_one_item in rucksack.0.chars() {
        for compartment_two_item in rucksack.1.chars() {
            if compartment_one_item == compartment_two_item {
                c = compartment_one_item;
                break 'outer;
            }
        }
    }
    if c == '0' {
        return 0;
    }
    calculate_priority(c)
}



fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    
    let mut special_values_sum: u32 = 0;

    for line in reader.lines() {
        let line = line?;

        let rucksack: Rucksack = line.split_at(line.len()/2);
        special_values_sum += get_common_value(rucksack);
    }

    println!("Sum: {}", special_values_sum);
    Ok(())
}
