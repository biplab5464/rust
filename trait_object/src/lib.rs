//created trait draw to used as trait object
pub trait Draw {
    fn Draw(&self);
}

// Definition of the Screen struct with a components field holding a vector of trait objects that implement the Draw trait
pub struct Screen(
    pub components: Vec<Box<dyn Draw>>,   //creating a dynamic variable of trait draw
)

//rendering the screen
impl Screen{
    pub fn run(&self){
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button{
    pub width : u32,
    pub height : u32,
    pub label : String,
}

impl Draw for Button{
    fn draw(&self){
        //actual code 
    }
}