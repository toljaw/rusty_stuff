fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &charachter) in bytes.iter().enumerate() {
        if charachter == b' ' {
            return &s[..i];
        }
    }

    return s;
}

fn main() {
    let hello = String::from("Rust ❤️");    // chars can have 1 - 4 bytes because os UTF 8
    // support - emojis can have 4 bytes and letters 1 byte - also can not call chars by index
    // because this reason

    println!("{}", hello);
    println!();

    let word = first_word(&hello);
    println!("First word: {}", word);
}