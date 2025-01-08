fn main() {
    // ASCII ranges:
    // 'A' to 'Z' → 65 to 90
    // 'a' to 'z' → 97 to 122

    for ascii in 65..=122 {
        let ch = ascii as u8 as char; // Convert ASCII to character
        if ch.is_ascii_alphabetic() {
            println!("ASCII: {}, Character: '{}'", ascii, ch);
        }
    }
}
