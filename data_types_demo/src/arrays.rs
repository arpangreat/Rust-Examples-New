use std::io;

pub fn arrays_demo() {
    let a = [1, 2, 3, 4, 5];

    let first = a[0];

    let second = a[1];

    println!("The values of first and second is: {}, {}", first, second);

    // Let's print the value from user input
    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    if index <= a.len() {
        println!(
            "The Value of the element at index {} is: {}",
            index, element
        );
    } else {
        eprintln!("The index should be in between 1 to 5");
    }
}
