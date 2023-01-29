fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    let mut s = String::from("hello world");
    let word;
    {
        let w = first_word(&s);
        word =w
    }
    println!("the first word is: {}", word);

    s.clear(); // error!
}
