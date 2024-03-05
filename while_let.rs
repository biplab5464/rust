fn main(){
    //Make 'Optional` of type `Option<i32>`
    let mut optional  = Some(0);

    //This reads: " while `let` destructures `optinal` into
    //`some(i)`, evaluate the block (`{}`). Else `Break`.
    while let Some(i) = optional  {
        if i > 9 {
            println!("Greater than 9, quti!");
            optional = None;
        } else {
            println!(" `i` is `{:?}`. Try again.",i);
            optional = Some(i+1);
        }
         // ^Less rightward drift drift and dosen't requre 
        // explicitly handlingthe failing case.
    }
    // ^ `if let` had additional optional `else`/`else if' 
    //clause. `while let` dose not have these.
   
}