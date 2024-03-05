fn main(){}

//if let is a shoter version of match that may have many codtion 
fn if_let(){
    let favourite_color : Option<&str> = None; //a option variable with None
    let is_tuesday  = false;
    let age: Result<u8,_> = "34".parse();   //trying ot parse string to intiger

    let Some(color) = favourite_color{  //here we are matching 'favorite_color' to 'option' (match favorite_color{ Some(color)=> "" , _ =>  } )
        println!("{}",color);
    } else if is_tuesday{
        println!("green");
    } else if let OK(age) = age{    //here we mathing age to result (match age{ oK(age ) => age, -=>})
        if age > 30{
            println!("purple");
        }else{
            println!("orange");
        }
    }else{
        println!("blue");
    }
}

//match expression inside a while ---while let
fn while_let(){
    let mut stack = vec![1,2,3];
    stack.push(4);

    // it will pop until evething is not 
    while let(top) = stack.pop(){
        println("{}",top)

    }
    /*
    while match {Some(top) => true, _ => false } {
        println!("{}",top);
    }
    */
}

fn for_loop(){
    let v = vec!['a','b','c'];

    for (index,value) in v.iter().enumerate(){
        println!("{} is at index {}", value, index);
    }

    //iter() just thought value
    // iter().enumerate() return a tupele wiht (index or value)
}

fn pattern_in_match_1(){
    let x = Some(5);
    let y = 20;
    
    match x {
        Some(50) => println!("Got 50"),  // this line will be false it will match the inside variable (50) 
        Some(y) => println("got y = {}",y),  //this will match as y will be generic and take any value inside Some that is 5
        _ => println("Default case, x = {:?}", x),  // this would have match if x was none and it would print None
    }
    //** the variable in Some(y) is not the same with the variable above it part of match block
}

fn patter_in_match_2(){
    let x =1;

    match x{
        1 | 2 => println!("one or two"),   // | - it kinda acts like 'or' 
        3 => println!("three"),
        _ => println!("Something else"),
    }

    let y = 5;

    match x {
        1..=5 => println!("one thought 5"),
        _ => println!("Something else"),
    }
}