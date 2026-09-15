fn create_dangling_reference() -> &i32 {
    let x = 10;
    &x
} //x gets destroyed here only bro stop here you cant point it then

fn main() {
    let reference = create_dangling_reference();

    println!("{}", reference);
}