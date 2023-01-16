// #![allow(dead_code)]
// #![allow(unused_variables)]
// #![allow(unused_mut)]
#![allow(dead_code, unused)]
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
}
