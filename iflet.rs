enum Foo{
    Bar,
    Baz,
    Qcx(u32)
}

fn main(){
    //Crating an example of the variable 
    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Foo::Qcx(100);

    //Variable a matches Foo::Bar
    if let Foo::Bar = a {
        println!("a is foobar");
    }

    //Variable b does not match Foo::Bar
    //So this will print nothing
    if let Foo::Bar = b{
        println!("b is foobar")
    }    

    //variablae c matches Foo::Qcx which has a value
    //Similar to Some() in the previous example
    if let Foo::Qcx(value) = c {
        println!(" c is {}" , value);
    }

    //Binding also works wiht `if let`
    // if let foo::(value @ 100) = c {
    //     println!("C is one hundred")
    // }
      
}