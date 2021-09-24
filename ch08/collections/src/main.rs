use std::collections::HashMap;

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    // create empty vector
    let a: Vec<i32> = Vec::new();

    // vec macro initiazes vector to hold 1, 2, 3
    let b = vec![1, 2, 3];

    // make it mutible to modify the contents
    let mut c = Vec::new();

    c.push(5);
    c.push(6);

    // Accessing Vector elements
    let first: &i32 = &b[0];
    println!("The first element is {}", first);

    match b.get(1) {
        Some(second) => println!("The second element is {}", second),
        None => println!("There is no second element."),
    }

    // looping over vectors
    let mut d = vec![100, 32, 57];
    for i in &d {
        println!("{}", i);
    }

    for i in &mut d {
        *i += 50;
    }

    for i in &d {
        println!("{}", i);
    }

    // Vector of enums to allow different types
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    // Strings
    // Create a new string
    let mut s = String::new();

    let data = "initial contents";

    // creating strings from string literals
    let t = data.to_string();
    let t = "initial contents".to_string();
    let t = String::from("initial contents");

    // appending onto a string
    let mut u = String::from("foo");
    u.push_str("bar");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // format macro doesn't take ownership of any arguments
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);

    // to access individual characters, use chars method
    for c in "hello world".chars() {
        println!("{}", c);
    }

    // Hashmaps
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 70);

    // Create hashmap by zipping two arrays together and calling collect()
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();

    // getting value from hashmap
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    // iterating over HashMap
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // check to see if there is an entry before overwriting value
    scores.entry(String::from("Red")).or_insert(60);
    scores.entry(String::from("Red")).or_insert(90);

    println!("{:?}", scores);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
