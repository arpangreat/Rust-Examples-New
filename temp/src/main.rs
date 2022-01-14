fn main() {
    let add = |x, y| x + y;
    println!("{}", add(1, 2));

    let v = vec![2, 4, 6];

    v.iter()
        .map(|x| x * 3)
        .filter(|x| *x > 10)
        .fold(0, |acc, x| acc + x);

    println!("{:?}", v);
}
