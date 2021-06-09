pub fn tuples_demo() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // destructing to assign the values of Tuple
    let (x, y, z) = tup;

    // printing the values
    println!("The values of x, y and z is: {}, {} and {}", x, y, z);

    // We can also assign Tuples values without destructing and only by period(.)
    let x1 = tup.0;

    let y1 = tup.1;

    let z1 = tup.2;

    println!(
        "This with period(.) method , The values of x, y and z is: {}, {} and {}",
        x1, y1, z1
    );
}
