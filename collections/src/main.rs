use std::collections::HashMap;
fn main (){
    //utf-8 encoding 
    let s:String = String::new();
    let s2 : &str = "new";
    let s3:String = s2.to_string();
    let s4:String = String::from("initial content");


    let blue :String = String::from("Blue");
    let mut scores :HashMap<String,i32> = HashMap::new();
    
    scores.insert(blue,20);
    let team_name: String = String::from("blue");
    let score : Option<&i32> = scores.get(&team_name);

    for(key, value) in &scores{
        println!(" {} : {}", key,value);
    }

}