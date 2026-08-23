fn main() {
    println!("Hello, world!");
    //another_function(23);
    let x = five();
    println!("{x}");
}

fn five()  -> i32{
    5
}
fn another_function(x:i32){
    println!("Value of x is {x}");
}
