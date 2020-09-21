use ::spellit::*;
use anyhow::{Context, Result};
use std::{
    env,
    io::{self, Read, Write},
};

fn main() -> Result<()> {
    let mut app = clap::App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .arg(
            clap::Arg::with_name("input")
                .required(true)
                .value_name("TEXT")
                .help("A literal text to spell in phonetic alphabet"),
        );

    let text = match app.get_matches_from_safe_borrow(env::args_os()) {
        Ok(matches) => (*matches.value_of_lossy("input").unwrap()).to_owned(),
        Err(error) => match error.kind {
            clap::ErrorKind::MissingRequiredArgument => {
                if atty::is(atty::Stream::Stdin) {
                    let stdout = io::stdout();
                    let mut stdout = stdout.lock();
                    app.write_help(&mut stdout)
                        .with_context(|| "Failed to write output to stdout.")?;
                    stdout
                        .write_all(b"\n\n")
                        .with_context(|| "Failed to write output to stdout.")?;
                    return Ok(());
                }
                let mut input = String::new();
                io::stdin()
                    .read_to_string(&mut input)
                    .with_context(|| "Failed to read input from stdin.")?;
                let input = input.trim().to_owned();
                input
            }
            _ => return Err(anyhow::Error::from(error)),
        },
    };

    let phonetics = phonetic_alphabets_of(&text);

    let characters = text.chars().zip(phonetics);
    for character in characters {
        let phonetic = character.1.unwrap_or("");
        println!("{} -> {}", character.0, phonetic);
    }

    Ok(())
}
