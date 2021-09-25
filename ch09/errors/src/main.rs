use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

fn main() {
    // simple panic to abort program
    // panic!("crash and burn");

    // using match to handle potential errors
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };

    // unwrap will return value on success or call panic! on failure
    let f = File::open("hello2.txt").unwrap();

    // expect can be better for debugging because of ability to set message
    let f = File::open("hello3.txt").expect("failed to open that file");
}

fn read_username_from_file(file: &str) -> Result<String, io::Error> {
    let f = File::open(file);

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file2(file: &str) -> Result<String, io::Error> {
    let mut f = File::open(file)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file3(file: &str) -> Result<String, io::Error> {
    let mut s = String::new();
    File::open(file)?.read_to_string(&mut s)?;

    Ok(s)
}
