mod slice_typing;

fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is: {}", s1, len);

    let s = String::from("Hello, World !!");

    let word = slice_typing::first_word(&s[..]);

    // s.clear();

    println!("{}", word);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
