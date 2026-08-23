fn main() {
    println!("Data typees in rust");
    let tup = (2,2.2,5);
    let (x,y,z) = tup;
    println!("the value of x is {x}");

    let arr: [i32; 5] = [1,2,5,4,4]; // [datatype;length of elements]
    let arr1 = [3;5]; // result would be [3,3,3,3,3]
}

