// Data Types in Rust
// Length	Signed	Unsigned
// 8-bit	i8	    u8
// 16-bit	i16	    u16
// 32-bit	i32	    u32
// 64-bit	i64	    u64
// 128-bit	i128	u128
// arch	    isize	usize

// bringing tuples in main file
mod tuples;
// bringing arrays in main file
mod arrays;

fn main() {
    let x = 2.6; // f64
    let y: f32 = 4.98123; // f32

    println!("The value of x and y is: {}, {}", x, y);

    // The Numeric Operations
    // addition
    let sum = 6 + 8;

    // difference
    let difference = 95.5 - 23.1;

    // multiplication
    let product = 4 * 30;

    // remainder
    let remainder = 34 % 5;

    // printing the values
    println!(
        "The value of sum, difference, product, remainder is: {}, {}, {}, {}",
        sum, difference, product, remainder
    );

    // Boolean Data Types
    let t = true;
    let f: bool = false;

    // printing the boolean types
    println!("The values of t and f is: {} and {}", t, f);

    // The Character Data Types
    let c = 'z';
    let name = "Swastik Acharyya";
    let red_heart = "❤️";

    // prinitng the values of Character Data Types
    println!(
        "The values of c , name and red_heart is: {}, {} and {}",
        c, name, red_heart
    );

    // The tuples data Types
    tuples::tuples_demo();

    // The Arrays Data Demo
    arrays::arrays_demo();
}
