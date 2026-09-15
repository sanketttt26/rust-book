struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: i32,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);


fn main ( ){

    //tuple-structs
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    let Color(x,y,z) = black;
    println!("{}",x);
}
