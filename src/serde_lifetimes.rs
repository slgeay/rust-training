use serde_json::{json, Value};

// pretend that we call an API and get a JSON String back
fn fetch_data() -> String {
    String::from(
        r#"
            {
                "id": 1,
                "title": "Hello, Rust"
            }
        "#,
    )
}

#[derive(Debug)]
struct BlogPost<'lifetime> {
    id: u32,

    title: &'lifetime str,
}

pub fn main() -> Result<(), serde_json::Error> {
    println!("<<< Serde Lifetimes >>>");
    #[allow(unused_assignments)]
    let mut title_mem = String::new();
    let post: BlogPost = {
        let data = fetch_data();
        let v: Value = serde_json::from_str(&data)?;
        title_mem = v["title"].as_str().unwrap().to_string();
        BlogPost { 
            id: v["id"].as_u64().unwrap() as u32, 
            title: title_mem.as_str()
        }
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