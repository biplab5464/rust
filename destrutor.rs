fn main(){


}

fn destrutring_structs(){
    struct Point{
        x : i32,
        y : i32,
    }

    let p = Point{
        x : 0,
        y : 7
    }; 
    let Point { x : a , y : b} = p; // it will destruct the varable to a and b 
    assert_eq(0,a);
    assert_eq(7,b);

    let Point { x , y } = p;  //it is equal to let Point {x : x, y : y} = p; 
}

fn destructing_enum(){
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    
    fn main() {
        let msg = Message::ChangeColor(0, 160, 255);
    
        match msg {
            Message::Quit => {
                println!("The Quit variant has no data to destructure.")
            }
            Message::Move { x, y } => {
                println!(
                    "Move in the x direction {} and in the y direction {}",
                    x, y
                );
            }
            Message::Write(text) => println!("Text message: {}", text),
            Message::ChangeColor(r, g, b) => println!(
                "Change the color to red {}, green {}, and blue {}",
                r, g, b
            ),
        }
        // you have use this before i am sure , don't you remember Some and None also Reuslt and Err, What really
    }
    
    //aslo you can use nested enum and you cand destrut them with ease as an example 
    Message::ChangeColor(Color::Hsv(h, s, v)) => println!(
        "Change the color to hue {}, saturation {}, and value {}",
        h, s, v
    )
    //note* this is part of a match
}

/////ignoring value in rust
fn ignoring_value(){
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);
    
}

fn ignoring_value_multiple(){
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {first}, {third}, {fifth}")
        }
    }
    
}

fn ignoring_value_struct(){
    
    struct Point {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point { x: 0, y: 0, z: 0 };

    match origin {
        Point { x, .. } => println!("x is {}", x),
    }
}

fn ignoring_value_tuple(){
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {first}, {last}");
        }
    }
}