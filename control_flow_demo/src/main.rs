fn main() {
    println!("Hello, world!");

    // Using if in a let statement
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);

    // Returning values from loops
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is: {}", result);

    // using While loop through a collection
    let a = [1, 2, 3, 4, 5, 6];
    let mut index = 0;

    while index < 5 {
        println!("The value is: {}", a[index]);

        index += 1;
    }

    // doing the same thing in for
    // IT'S MORE SAFE BTW
    for element in a.iter() {
        println!("The value of a is: {}", element);
    }
    --
}
