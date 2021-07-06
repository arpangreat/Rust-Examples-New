mod hashmap;
mod utf8_and_strings;
mod vectors;

fn main() {
    println!("From The Vectors Module");
    vectors::vectors();
    println!(" ");
    println!("From The UTF-8 And Strings Module");
    utf8_and_strings::run();
    println!(" ");
    println!("From The HashMap Module");
    hashmap::hashmap();
}
