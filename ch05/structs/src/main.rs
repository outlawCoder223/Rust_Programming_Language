struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Color(i32, i32, i32);

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, test_rect: &Rectangle) -> bool {
        self.height > test_rect.height && self.width > test_rect.width
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let user1 = User {
        email: String::from("picklerick@citadel.com"),
        username: String::from("impicklerick"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("evilmorty@citadel.com"),
        username: String::from("notevilmorty"),
        ..user1
    };

    let user3 = build_user(String::from("summer@citadel.com"), String::from("summer"));

    let black = Color(0, 0, 0);

    let rect1 = Rectangle {
        width: 30,
        height: 70,
    };

    let rect2 = Rectangle {
        width: 20,
        height: 20,
    };

    let rect3 = Rectangle {
        width: 40,
        height: 40,
    };

    println!("rect1 is {:?}", rect1);
    println!("The area of the rectagle is {}", rect1.area());
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
