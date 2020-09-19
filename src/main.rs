const PHONETIC_ALPHABETS: [&str; 26] = [
    "Alfa", "Bravo", "Charlie", "Delta", "Echo", "Foxtrot", "Golf", "Hotel", "India", "Juliet",
    "Kilo", "London", "Mike", "November", "Oscar", "Papa", "Quebec", "Romeo", "Sierra", "Tango",
    "Uniform", "Victor", "Whiskey", "X-ray", "Yankee", "Zulu",
];

fn phonetic_alphabet_of(text: &str) -> Vec<Option<&str>> {
    text.chars()
        .map(|c| {
            let d = c as u8;
            if d >= 97 && d <= 122 {
                return Some(PHONETIC_ALPHABETS[(d - 97) as usize]);
            }
            if d >= 65 && d <= 90 {
                return Some(PHONETIC_ALPHABETS[(d - 65) as usize]);
            }
            None
        })
        .collect()
}

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
    let phonetics = phonetic_alphabet_of(&text);

    let characters = text.chars().zip(phonetics);
    for character in characters {
        let phonetic = character.1.unwrap_or("");
        println!("{} -> {}", character.0, phonetic);
    }

    Ok(())
}
