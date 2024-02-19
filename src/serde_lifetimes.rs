use serde_json::json;
use std::borrow::Cow;


// pretend that we call an API and get a JSON String back
fn fetch_data() -> String {
    String::from(
        r#"
            {
                "id": 1,
                "title": "Hello, \"Rust\""
            }
        "#,
    )
}

#[derive(Debug, serde::Deserialize)]
struct BlogPost<'lifetime> {
    id: u32,

    title: Cow<'lifetime, str>,
}

pub fn main() -> Result<(), serde_json::Error> {
    println!("<<< Serde Lifetimes >>>");
    
    let data = fetch_data();
    let post: BlogPost = {
        let v = serde_json::from_str(&data)?;
        v
    };
    println!("deserialized = {:?}", post);

    let post_json: String =
        serde_json::to_string(
            &json!(
                {
                    "id": post.id,
                    "title": post.title
                }
            )
        )?
    ;

    println!("serialized = {:?}", post_json);

    Ok(())
}