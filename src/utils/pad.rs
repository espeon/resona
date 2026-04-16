use crate::utils::pick;

const PADDING_SYLLABLES: &[&str] = &[
    "a", "o", "u", "i", "e", "ra", "ro", "ri", "re", "no", "na", "ni", "ne", "li", "la", "lo",
    "le", "ka", "ko", "ki", "ke", "ta", "to", "ti", "te", "sa", "so", "si", "se", "ma", "mo", "mi",
    "me", "da", "do", "di", "de", "ba", "bo", "bi", "be", "pa", "po", "pi", "pe", "xa", "xo", "xi",
    "xe",
];

pub fn pad_to_len(s: &str, min: usize, max: usize, rng: &mut nanorand::ChaCha<8>) -> String {
    if s.len() >= max {
        return s[..s.floor_char_boundary(max)].to_string();
    }

    if s.len() >= min {
        return s.to_string();
    }
    let mut result = String::with_capacity(max);
    result.push_str(s);

    while result.len() < min {
        let syllable = pick(PADDING_SYLLABLES, rng);
        result.push_str(syllable);
    }

    if result.len() > max {
        result.truncate(result.floor_char_boundary(max));
    }

    result
}
