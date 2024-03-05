use::std::collections::HashMap;

fn main(){
    let mut score = HashMap::new();

    score.insert("blue".to_string(),"nope");
    score.insert("yellow".to_string(),"nice");

    score.entry("grey".to_string()).or_insert("again");
    score.entry("yellow".to_string()).or_insert("nothing");
     
    //println!("{:?}", score);

    for (key,value) in score.iter(){
        println!("key- {} : value - {} ",key,value)
    }
}
