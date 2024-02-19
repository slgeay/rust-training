use serde_json::json;
use std::borrow::Cow;


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

#[derive(Debug, serde::Deserialize)]
struct BlogPost<'lifetime> {
    id: u32,
    #[serde(borrow)]
    title: Cow<'lifetime, str>,
}

pub fn main() -> Result<(), serde_json::Error> {
    println!("<<< Serde Lifetimes >>>");
    
    let data = fetch_data();
    let post: BlogPost = {
        let v = serde_json::from_str(&data)?;
        v
    };

    println!("is borrowed = {}", matches!(post.title, Cow::Borrowed(_)));
        
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

#[test]
fn test_borrowed() {
    main().unwrap();

    let data = String::from(
        r#"
            {
                "id": 1,
                "title": "Hello, Rust"
            }
        "#,
    );

    let post: BlogPost = serde_json::from_str(&data).unwrap();
    assert!(matches!(post.title, Cow::Borrowed(_)));
}

#[test]
fn test_owned() {
    main().unwrap();

    let data = String::from(
        r#"
            {
                "id": 1,
                "title": "Hello, \"Rust\""
            }
        "#,
    );

    let post: BlogPost = serde_json::from_str(&data).unwrap();
    assert!(!matches!(post.title, Cow::Borrowed(_)));
}

#[test]
fn test_serialized() {
    main().unwrap();

    let data = String::from(
        r#"
            {
                "id": 1,
                "title": "Hello, Rust"
            }
        "#,
    );

    let post: BlogPost = serde_json::from_str(&data).unwrap();
    let post_json: String = serde_json::to_string(&json!({"id": post.id, "title": post.title})).unwrap();
    assert_eq!(post_json, r#"{"id":1,"title":"Hello, Rust"}"#);
}

#[test]
fn test_serialized_escaped() {
    main().unwrap();

    let data = String::from(
        r#"
            {
                "id": 1,
                "title": "Hello, \"Rust\""
            }
        "#,
    );

    let post: BlogPost = serde_json::from_str(&data).unwrap();
    let post_json: String = serde_json::to_string(&json!({"id": post.id, "title": post.title})).unwrap();
    assert_eq!(post_json, r#"{"id":1,"title":"Hello, \"Rust\""}"#);
}