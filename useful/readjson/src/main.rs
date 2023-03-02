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
    let json = r#"{
        "article": "how to work with json in rust",
        "author": "me",
        "paragraph": [
            {
                "name": "starting sentence"
            },
            {
                "name": "body of the paragraph"
            },
            {
                "name": "end of the paragraph"
            },
        {
            "name": "paragraph"
        }
        ]
    }"#;
let parsed: Article = read_json_type(json);
println!("\n\n The name of the first paragraph is {}", parsed.paragraph[0].name);
}
fn read_json_type(raw_json: &str) -> Article {
    let parsed: Article = serde_json::from_str(raw_json).unwrap();
    return parsed;
}