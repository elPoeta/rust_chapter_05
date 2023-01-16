// #![allow(dead_code)]
// #![allow(unused_variables)]
// #![allow(unused_mut)]
#![allow(dead_code, unused)]
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }
}

fn area(r: &Rectangle) -> u32 {
    r.width * r.height
}
fn main() {
    {
        let mut user1 = User {
            email: String::from("someone@example.com"),
            username: String::from("someusername123"),
            active: true,
            sign_in_count: 1,
        };

        user1.email = String::from("anotheremail@example.com");

        let user2 = User {
            email: String::from("another@example.com"),
            ..user1
        };
        println!("{:#?}", user2);
    }

    {
        let mut user = build_user(
            String::from("leonardo.a.tosetto@gmail.com"),
            String::from("elPoeta"),
        );
    }
    {
        let black = Color(0, 0, 0);
        let origin = Point(0, 0, 0);
    }

    {
        let rect = Rectangle {
            width: 30,
            height: 15,
        };
        println!("{:#?}", rect);
        println!("Area: {}", area(&rect));
        println!("Area from method: {}", rect.area());
        dbg!(&rect);
    }
    {
        let sq = Rectangle::square(3);
        println!("{:#?}", sq);
    }
}
