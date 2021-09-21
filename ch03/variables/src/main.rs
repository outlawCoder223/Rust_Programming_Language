fn main() {
    // example of constant
    const MAX_POINTS: u32 = 100_00;

    // must use "mut" to make a variable mutable
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 7;
    println!("The value of x is :{}", x);

    // example of shadowing variable
    let y = 5;
    let y = y + 1;
    let y = y * 2;
    println!("The value of y is: {}", y);

    // shadowing allows for reuse of names if types differ
    let spaces = "    ";
    let spaces = spaces.len();

    // Rust primitive types:
    let integer: i32 = 56;
    let float: f64 = 30.12;
    let boolean: bool = true;
    let character: char = 'R';

    // Rust compound types:
    // 1. Tuple
    let tup: (i32, f64, u8) = (500, 5.4, 1);
    // Tuple destructuring to get values
    let (a, b, c) = tup;
    println!("The values of a, b and c are: {}, {}, {}", a, b, c);
    // can use "." to get values as well
    let a = tup.0;
    let b = tup.1;
    let c = tup.2;
    println!("The values of a, b and c are {}, {}, {}", a, b, c);

    // 2. Array
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let first = arr[0];
    let second = arr[2];

    another_function(a);
    let my_return = add_one(arr[2]);

    // classic if/else
    let number = 3;

    if number < 5 {
        println!("very true!");
    } else {
        println!("Very false!");
    }

    // looping
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("{}", result);

    counter = 0;

    while counter < 4 {
        println!("{}", counter);

        counter += 1;
    }

    for el in arr.iter() {
        println!("the value is: {}", el);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!");
}

fn another_function(x: i32) {
    println!("The value of x is: {}.", x);
}

fn add_one(num: i32) -> i32 {
    num + 1
}
