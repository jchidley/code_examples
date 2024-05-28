//! Serde array of arrays.
//! [Use of Serde's #[serde(transparent)]](https://stackoverflow.com/questions/78535051/use-of-serdes-serdetransparent)
//! [Struct expressions](https://doc.rust-lang.org/reference/expressions/struct-expr.html)
//! [The Tuple Type](https://doc.rust-lang.org/book/ch03-02-data-types.html#the-tuple-type)
//! [Tuple Structs](https://doc.rust-lang.org/book/ch05-01-defining-structs.html#using-tuple-structs-without-named-fields-to-create-different-types)
//! [Empty Tuple Structs](https://rust-lang.github.io/rfcs/0218-empty-struct-with-braces.html#empty-tuple-structs)

//! [1Password Developer Fireside Chat: Serde Deserializers](https://www.youtube.com/watch?v=7pZTYdqXfgY)
//! [Parsing JSON in Rust using serde and reqwest](https://www.youtube.com/watch?v=ogpE4hviXyA)
//! [Rust Linz, November 2021 - Serde Shenanigans by Armin Ronacher](https://www.youtube.com/watch?v=UhZGYS13twc)
use chrono::{DateTime, Utc};
use serde::{Deserialize};
use serde_json::{self, Result};
use serde_with::formats::Flexible;
use serde_with::TimestampSeconds;

const EMONCMS_FEED_DATA: &str = r#"
[
  [
    1716705669,
    272430
  ],
  [
    1716705729,
    272436
  ]
]"#;

#[serde_with::serde_as]
#[derive(Deserialize, Debug)]
struct MsgTuple(
    #[serde_as(as = "TimestampSeconds<String, Flexible>")] DateTime<Utc>,
    i32,
);

#[derive(Deserialize, Debug)]
struct EmoncmsMsgTuple(Vec<MsgTuple>);

impl EmoncmsMsgTuple {
    pub fn new(data: &str) -> Result<EmoncmsMsgTuple> {
        serde_json::from_str(data)
    }
}

fn main() {
    let table: EmoncmsMsgTuple = EmoncmsMsgTuple::new(EMONCMS_FEED_DATA).unwrap();
    println!("{:?}", table);
// [serde_json](https://docs.rs/serde_json/latest/serde_json/index.html)
// [Rust: Serde: Working with untyped JSON values](https://www.youtube.com/watch?v=NwYY00paiH0)
    let serde_data: serde_json::Value = serde_json::from_str(EMONCMS_FEED_DATA).unwrap();
    println!("{:?}", serde_data);
}

