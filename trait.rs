struct NewsArticle{
    headline : String,
    location : String,
    author : String,
    content: String,
}

struct Tweet{
    username : String,
    content : String,
    reply :  bool,
    retweet : bool,
}

trait summary{
    fn summarize(&self) -> String{
        format!("we have no imforamtion")
    }
}

 impl summary for NewsArticle{
//     fn summarize(&self) -> String{
//         format!("{}, by {} ({})",self.headline,self.author,self.location)
//     }
 }

impl summary for Tweet{
    fn summarize(&self) -> String{
        format!("{}: {}",self.username,self.content)
    }
}

fn main(){
    let tweet = Tweet{
        username : String::from("@baymax5464"),
        content : String::from("yeah it is my first Tweet,hehehe"),
        reply : false,
        retweet : false,
    };
    let news = NewsArticle{
        headline : String::from("someone got steel glass in rectum"),
        location : String::from("Bhadrak"),
        author :  String::from("Biplab Gartia"),
        content : String::from("Returning either a NewsArticle of a Tweet isn't allowed due to restriciton around how the impl Trait syntax is implemeted syntax is implemted in the compiler. We'll cover how to write how to write a fuction with the behaviour in the \"Using Trait objects That allwo for values of Diffrent Types\" Section of Chpater 17."), 
    };
    println!("1 new tweet: {}", tweet.summarize());
    println!("1 new headline {}", news.summarize());
}
