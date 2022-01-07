use std::thread;

fn main() {
    let mut a = vec![1, 2, 3];
    let mut x = 0;

    thread::scope(|s| {
        s.spawn(|_| {
            println!("Hello from first thread");

            dbg!(&a);
        })
    });

    a.push(2);
    assert_eq!(x, a.len());
}
