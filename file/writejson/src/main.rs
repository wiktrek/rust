use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
struct Paragraph {
    name: String,
}

#[derive(Serialize, Deserialize)]
struct Article {
    article: String,
    author: String,
    paragraph: Vec<Paragraph>,
}
fn main() {
    let article: Article = Article {
        article: String::from("Rust write json"),
        author: String::from("wiktrek"),
        paragraph: vec![
            Paragraph {
                name: String::from("name 1"),
            },
            Paragraph {
                name: String::from("name 2"),
            },
            Paragraph {
                name: String::from("name 3"),
            },
        ],
    };

    let json = serde_json::to_string(&article).unwrap();

    println!("the JSON is: {}", json)
}
