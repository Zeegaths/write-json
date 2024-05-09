use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Paragraph {
    name:String
}

#[derive(Serialize, Deserialize)]
struct Article {
    article: String,
    author: String,
    paragraph: Vec<Paragraph>
}



fn main() {
    let article: Article = Article{
        article: String::from("How to make learning rust fun"),
        author: String::from("Mary"),
        paragraph: vec![
            Paragraph {
                name: String::from("My First Paragraph")
            },
            Paragraph{
                name:String::from("My Second Paragraph")
            },
            Paragraph{
                name:String::from("My Third Paragraph")
            }
        ]
    };

    let json = serde_json::to_string(&article).unwrap();
    println!("the json is: {}", json)
}