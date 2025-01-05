use color_eyre::*;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let input = include_str!("../input.txt");
    let mut lines = input.lines();

    for line in lines {
        let token = line.split_whitespace();

        let nums = token.iter().map(|s| s.parse::<u32>()).collection();
        
    }
    Ok(())
}
