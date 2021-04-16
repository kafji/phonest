// Based on table in https://en.wikipedia.org/wiki/NATO_phonetic_alphabet#Pronunciation_of_code_words

const PHONETIC_LETTERS: [&str; 26] = [
    "Alfa", "Bravo", "Charlie", "Delta", "Echo", "Foxtrot", "Golf", "Hotel", "India", "Juliet",
    "Kilo", "London", "Mike", "November", "Oscar", "Papa", "Quebec", "Romeo", "Sierra", "Tango",
    "Uniform", "Victor", "Whiskey", "X-ray", "Yankee", "Zulu",
];

const PHONETIC_NUMBERS: [&str; 10] = [
    "Zero", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine",
];

#[inline]
pub fn phonetic_alphabets_of(text: &str) -> Vec<Option<&str>> {
    text.chars()
        .map(|c| {
            let d = c as u8;
            // Numbers
            if d >= 48 && d <= 57 {
                return Some(PHONETIC_NUMBERS[(d - 48) as usize]);
            }
            // Non-capitalized letters
            if d >= 97 && d <= 122 {
                return Some(PHONETIC_LETTERS[(d - 97) as usize]);
            }
            // Capitalized letters
            if d >= 65 && d <= 90 {
                return Some(PHONETIC_LETTERS[(d - 65) as usize]);
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
        let txt = "1234";
        let phonetic = phonetic_alphabets_of(txt);
        assert_eq!(
            phonetic,
            ["One", "Two", "Three", "Four"]
                .iter()
                .cloned()
                .map(|x| Some(x))
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_ascii_symbols() {
        let txt = "!@#";
        let phonetic = phonetic_alphabets_of(txt);
        assert_eq!(phonetic, (0..3).map(|_| None).collect::<Vec<_>>());
    }
}
