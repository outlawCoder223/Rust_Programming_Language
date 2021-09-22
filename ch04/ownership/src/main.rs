fn main() {
    let _s = "hello"; // string literal stored on stack
    let mut s = String::from("hello"); // string stored on heap

    s.push_str(", world!");

    println!("{}", s); // This will print 'hello, world'

    // moves string pointer to s2, taking ownership
    let s1 = String::from("Rancey");
    let s2 = s1;

    // can't do this
    // println!("{}, pants!", s1);

    // s2 clones s1 making it's own copy
    let s1 = String::from("Rancey");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    let s = String::from("Hi there"); // s comes into scope

    takes_ownership(s); // s's value moves into the function..
                        // ... and is no longer valid here

    let x = 5; // x comes into scope

    makes_copy(x); // x would move into teh function,
                   // but i32 is Copy, so it's okay to still
                   // use x afterward

    let s1 = String::from("Boo yah");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}", s1, len);

    let mut s = String::from("hello");

    change(&mut s);

    println!("{}", s);

    // Slices
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..];
    let mut s = String::from("hello world");

    let word = first_word(&s);

    println!("the first word is: {}", word);

    s.clear();
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}
