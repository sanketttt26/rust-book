#[derive(Debug)]
struct Rectangle {
    width : u32,
    breadth : u32,
}

fn main() {
    let rec1 = Rectangle{
        width : 10,
        breadth : 10,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rec1)
    );
    println!("rec1 is {rec1:?}");
}



//area just borrows the rec struct performs actions and returns 
fn area(rec : &Rectangle) -> u32 {
    rec.width * rec.breadth
}