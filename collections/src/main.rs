fn main() {
    let a: [i32;_] = [1,2,3];
    let mut v:Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    {
        let v2:Vec<i32> = vec![1,2,4];
    }

    //accessing the vectors 
    let mut v3:Vec<i32> = vec![1,2,4,4,5,5];
    let third : &i32 = &v3[2];
    println!("the num at index 3 is {}",third);

    match v3.get(2){
        Some(third) => println!("the num at index 3 is {}",third),
        None => println!("no index found "),
    }

    //looping in vec
    for i in &mut v3 {
        *i+=50;
    }

    for i in &v3 {
        println!(" {}",i);
    }
}
