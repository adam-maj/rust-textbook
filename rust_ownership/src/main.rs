use std::io;

fn first_word_index(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    return s.len();
}

fn first_word_slice(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    return &s[..];
}

fn main() {
    let mut sentence = String::new();
    
    println!("Please input a sentence.");
    match io::stdin().read_line(&mut sentence) {
        Err(error) => panic!("Error reading from stdin: {}", error),
        Ok(_) => {}
    };

    let index = first_word_index(&sentence);
    println!("(first_word_index): {}", &sentence[0..index]);

    let slice = first_word_slice(&sentence);
    println!("(first_word_slice): {}", slice);
}
