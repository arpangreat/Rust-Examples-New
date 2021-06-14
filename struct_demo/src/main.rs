mod methods_demo;
mod rectangles;

fn main() {
    // defining Structs
    // struct User {
    //     username: String,
    //     email: String,
    //     number: u64,
    //     active: bool,
    // }

    // creating a User Struct
    // HACK: making a build fn to make it more dynamic
    // fn build_user(email: &str, username: &str) -> User {
    //     User {
    //         // username: username,
    //         // email: email,
    //         number: 90909098,
    //         active: true,
    //     }
    // }
    //
    // let user1 = build_user("arpanmail", "swastik");
    //
    // // printing the values
    // println!("{}", user1.username);
    // println!("{}", user1.email);
    // println!("{}", user1.number);
    // println!("{}", user1.active);

    rectangles::run();
}
