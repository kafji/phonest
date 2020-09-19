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
