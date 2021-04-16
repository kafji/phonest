const PHONETIC_ALPHABETS: [&str; 26] = [
    "Alfa", "Bravo", "Charlie", "Delta", "Echo", "Foxtrot", "Golf", "Hotel", "India", "Juliet",
    "Kilo", "London", "Mike", "November", "Oscar", "Papa", "Quebec", "Romeo", "Sierra", "Tango",
    "Uniform", "Victor", "Whiskey", "X-ray", "Yankee", "Zulu",
];

#[inline]
pub fn phonetic_alphabets_of(text: &str) -> Vec<Option<&str>> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ascii_letters() {
        let txt = "abcxyz";
        let phonetic = phonetic_alphabets_of(txt);
        assert_eq!(
            phonetic,
            ["Alfa", "Bravo", "Charlie", "X-ray", "Yankee", "Zulu"]
                .iter()
                .cloned()
                .map(|x| Some(x))
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_ascii_numbers() {
        let txt = "1337";
        let phonetic = phonetic_alphabets_of(txt);
        assert_eq!(phonetic, (0..4).map(|_| None).collect::<Vec<_>>());
    }

    #[test]
    fn test_ascii_symbols() {
        let txt = "!@#";
        let phonetic = phonetic_alphabets_of(txt);
        assert_eq!(phonetic, (0..3).map(|_| None).collect::<Vec<_>>());
    }
}
