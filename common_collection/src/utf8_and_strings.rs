// INFO: There is a number of other string types in Rust Standard Library
// Such as: OsString, OsStr, CString, CStr

pub fn run() {
    let mut s = String::new();
    s.push_str("Rust");
    println!("{}", s);

    let data = "Hello, There";
    let s = data.to_string();
    println!("{}", s);

    // NOTE: This Also Works
    let s = "Rust is The Best".to_string();
    println!("{}", s);

    let s1 = String::from("Ayyy, Yooo !!");
    println!("{}", s1);

    let ascii = "Swastik";
    let swastik = ascii.to_ascii_uppercase();
    println!("{}", swastik);

    let str1 = String::from("Swastik is ");
    let str2 = String::from("The Best Programmer");
    let str3 = str1 + &str2; // NOTE: s1 has been moved here and can no longer be used
    println!("{}", str3);

    let v = 32;
    let str_v = ToString::to_string(&v);
    let str_v_macro = stringify!(&v); // NOTE: We can use this macro to stringify any data type
    println!("{}", str_v);
    println!("{}", str_v_macro);

    // Slicing Strings
    let hello = "Здравствуйте";

    let s_hello = &hello[0..4];
    println!("{}", s_hello);

    // Methods For Iterating Over String
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    // Using Bytes
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}

/* NOTE:
* The string str3 will contain Swastik is The Best Programmer as a result of this code. The reason
* str1 is no longer
* valid after the addition and the reason we used a reference to str2 has to do with the signature
* of
* the method that gets called when we use the + operator. The + operator uses the add method, whose
* signature looks something like this:
*
*
* fn add(self, s: &str) -> String {
*
* This isn’t the exact signature that’s in the standard library: in the standard library, add is
* defined using generics. Here, we’re looking at the signature of add with concrete types
* substituted for the generic ones, which is what happens when we call this method with String
* values. We’ll discuss generics in Chapter 10. This signature gives us the clues we need to
* understand the tricky bits of the + operator.
*
* First, str2 has an &, meaning that we’re adding a reference of the second string to the first
* string because of the s parameter in the add function: we can only add a &str to a String; we
* can’t add two String values together. But wait—the type of &str2 is &String, not &str, as
* specified
* in the second parameter to add. So why does Listing 8-18 compile?
*
* The reason we’re able to use &str2 in the call to add is that the compiler can coerce the &String
* argument into a &str. When we call the add method, Rust uses a deref coercion, which here turns
* &str2 into &str2[..]. We’ll discuss deref coercion in more depth in Chapter 15. Because add does
* not
* take ownership of the s parameter, str2 will still be a valid String after this operation.
*
* Second, we can see in the signature that add takes ownership of self, because self does not have
* an &. This means str1 in Listing 8-18 will be moved into the add call and no longer be valid
* after
* that. So although let str3 = str1 + &str2; looks like it will copy both strings and create a new
* one,
* this statement actually takes ownership of str1, appends a copy of the contents of str2, and then
* returns ownership of the result. In other words, it looks like it’s making a lot of copies but
* isn’t; the implementation is more efficient than copying. */

/* NOTE:
 * Bytes and Scalar Values and Grapheme Clusters! Oh My!
 * Another point about UTF-8 is that there are actually three relevant ways to look at strings from
* Rust’s perspective: as bytes, scalar values, and grapheme clusters (the closest thing to what we
* would call letters).
 *
 * If we look at the Hindi word “नमस्ते” written in the Devanagari script, it is stored as a vector
* of u8 values that looks like this:
 *
 *
 * [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164,
 * 224, 165, 135]
*
 * That’s 18 bytes and is how computers ultimately store this data. If we look at them as Unicode
* scalar values, which are what Rust’s char type is, those bytes look like this:
 *
 *
 * ['न', 'म', 'स', '्', 'त', 'े']
*
 * There are six char values here, but the fourth and sixth are not letters: they’re diacritics
* that don’t make sense on their own. Finally, if we look at them as grapheme clusters, we’d get
* what a person would call the four letters that make up the Hindi word:
 *
 *
 * ["न", "म", "स्", "ते"]
*
 * Rust provides different ways of interpreting the raw string data that computers store so that
* each program can choose the interpretation it needs, no matter what human language the data is
* in.
 *
 * A final reason Rust doesn’t allow us to index into a String to get a character is that indexing
* operations are expected to always take constant time (O(1)). But it isn’t possible to guarantee
* that performance with a String, because Rust would have to walk through the contents from the
* beginning to the index to determine how many valid characters there were. */
