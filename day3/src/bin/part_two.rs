fn calculate_priority(c: char) -> u32 {
    match c {
        'a'..='z' => c as u32 - 'a' as u32 + 1,
        'A'..='Z' => c as u32 - 'A' as u32 + 27,
        _ => panic!("wtf not a valid letter"),
    }
}

fn main() -> std::io::Result<()> {
    let input = std::fs::read_to_string("input.txt")?;

    let groups = input.lines().collect::<Vec<&str>>();

    let result = groups.chunks(3).fold(0, |sum, group| {
        let (first, second, third) = (group[0], group[1], group[2]);
        let type_badge = first
            .chars()
            .find(|&type_badge| second.contains(type_badge) && third.contains(type_badge))
            .unwrap();
        sum + calculate_priority(type_badge)
    });

    println!("{}", result);

    Ok(())
}
