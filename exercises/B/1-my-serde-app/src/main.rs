//! Adapted from https://github.com/ferrous-systems/teaching-material/blob/main/assignments/serde-lifetimes.adoc

use serde::Deserialize;

/// pretend that we call an API and get a JSON String back
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
struct BlogPost {
    id: u32,
    title: String,
}

fn main() -> anyhow::Result<()> {
    let post: BlogPost = serde_json::from_str(fetch_data().as_str()).unwrap();

    println!("deserialized = {:?}", post);

    let post_json: String = serde_json::to_string(&post.into()).unwrap();
    println!("serialized = {:?}", post_json);

    Ok(())
}
