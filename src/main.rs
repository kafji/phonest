use ::spellit::*;

fn main() -> anyhow::Result<()> {
    let matches = clap::App::new(env!("CARGO_PKG_NAME"))
        .setting(clap::AppSettings::ArgRequiredElseHelp)
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .arg(
            clap::Arg::with_name("input")
                .required(true)
                .value_name("TEXT")
                .help("A literal text to spell in phonetic alphabet"),
        )
        .get_matches();

    let text = matches.value_of_lossy("input").unwrap();
    let phonetics = phonetic_alphabets_of(&text);

    let characters = text.chars().zip(phonetics);
    for character in characters {
        let phonetic = character.1.unwrap_or("");
        println!("{} -> {}", character.0, phonetic);
    }

    Ok(())
}
