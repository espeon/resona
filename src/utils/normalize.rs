pub fn normalize_string(s: &str) -> String {
    let nfkd = unicode_normalization::UnicodeNormalization::nfkd(s);
    nfkd.filter(|c| c.is_ascii_alphanumeric()).collect()
}
