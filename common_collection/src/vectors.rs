pub fn vectors() {
    let mut v: Vec<i32> = Vec::new();

    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);

    let third: &i32 = &v[2];
    println!("The Third element is {}", third);

    let v1 = vec![1, 2, 3, 4];
    match v1.get(2) {
        Some(third_v1) => println!("The Third element is {}", third_v1),
        None => println!("There is no third Element"),
    }

    /* BUG: This will not work
    let first = &v[0];
    * v.push(6);
    * println!("The first element is {}", first); */

    // HACK: How Ever This Works
    v.push(6);
    let first = &v[0];
    println!("The first element is {}", first);

    // NOTE: Iterating over the values of The Vector
    for i in &v {
        println!("The Values of V are [{}]", i);
    }
    // More Advanced
    for i in &mut v {
        *i += 50;
        println!("The Values are {}", i);
    }

    // HACK: Using an Enum To Store Multiple Values
    enum SpreadSheetCell {
        Int(i32),
        Float(f32),
        Text(String),
    }

    let row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Float(10.12),
        SpreadSheetCell::Text(String::from("Rust")),
    ];

    // BUG: Fix this !!
    /* for i in &row {
     *     println!("The Values of row: {:?}", i);
     * } */
}
