// HACK : refactoring
// TODO: make a separate file for methods demo
#[derive(Debug)]
struct Rectangle {
    width: u32,
    heigth: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.heigth * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.heigth > other.heigth
    }
}

pub fn run() {
    let rect1 = Rectangle {
        width: 30,
        heigth: 30,
    };
    let rect2 = Rectangle {
        width: 20,
        heigth: 10,
    };
    let rect3 = Rectangle {
        width: 10,
        heigth: 60,
    };

    println!(
        "The area of the rectangle is {} square pixels",
        // area(&rect1) // to see rect data we have to put {:?} instead
        rect1.area()
    );

    println!("Can rect1 hold rect2??  {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3??  {}", rect1.can_hold(&rect3));
}

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.heigth
// }
