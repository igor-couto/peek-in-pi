use std::error::Error;
use std::io::{self, Write};

pub fn get_player_input() -> String {
    let stdin = io::stdin();

    loop {
        match try_get_input(&stdin) {
            Ok(value) => return value,
            Err(error) => {
                println!("{:?}", error);
                continue;
            }
        };
    }
}

fn try_get_input(stdin: &io::Stdin) -> Result<String, Box<dyn Error>> {
    print!("\nEnter a number: ");
    io::stdout().flush()?;

    let mut buffer = String::new();
    stdin.read_line(&mut buffer)?;

    let input = &buffer.trim_end();

    input.parse::<u32>()?;

    Ok(String::from(input.to_owned()))
}
