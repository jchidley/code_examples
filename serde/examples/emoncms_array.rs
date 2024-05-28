//! Serde array of arrays.
//! [](https://stackoverflow.com/questions/59873674/rust-deserialize-a-json-array-into-a-very-simple-custom-table)
//! [](https://stackoverflow.com/questions/61831962/deserializing-a-datetime-from-a-string-millisecond-timestamp-with-serde)
//! [EmonCMS API Documentation](https://emoncms.org/site/api) and [Feed API](https://emoncms.org/site/api#feed)
//! [EmonCMS Documentation](https://github.com/emoncms/emoncms/tree/master/docs)
//! [](https://doc.rust-lang.org/nomicon/other-reprs.html)
//! [Use of Serde's #[serde(transparent)]](https://stackoverflow.com/questions/78535051/use-of-serdes-serdetransparent)
//! [Newtype](https://rust-unofficial.github.io/patterns/patterns/behavioural/newtype.html)
//! [New Type Idiom](https://doc.rust-lang.org/rust-by-example/generics/new_types.html)
//! [Embrace the newtype pattern](https://www.lurklurk.org/effective-rust/newtype.html)
//! [The Rust Playground](https://play.rust-lang.org/help)
//! [mdBook](https://github.com/rust-lang/mdBook) and [mdBook Documentation](https://rust-lang.github.io/mdBook/)

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
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
pub struct Msg {
    #[serde_as(as = "TimestampSeconds<String, Flexible>")]
    pub date_time: DateTime<Utc>,
    pub msg: i32,
}

#[derive(Deserialize, Debug)]
#[serde(transparent)]
pub struct EmoncmsMsg {
    pub rows: Vec<Msg>,
}

impl EmoncmsMsg {
    pub fn new(data: &str) -> Result<EmoncmsMsg> {
        serde_json::from_str(data)
    }
}

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

#[derive(Debug, Deserialize, Serialize)]
#[serde(transparent)]
struct Transparent {
    foo: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Intransparent {
    foo: String,
}

fn main() {
    let trans = Transparent {
        foo: "hello transparent".into(),
    };
    dbg!(&trans);
    let _ = dbg!(serde_json::to_string(&trans));
    let intrans = Intransparent {
        foo: "hello intransparent".into(),
    };
    dbg!(&intrans);
    let _ = dbg!(serde_json::to_string(&intrans));

    let table: EmoncmsMsg = EmoncmsMsg::new(EMONCMS_FEED_DATA).unwrap();
    assert_eq!(table.rows.len(), 2);
    println!("{:?}", table);

    let table: EmoncmsMsgTuple = EmoncmsMsgTuple::new(EMONCMS_FEED_DATA).unwrap();
    println!("{:?}", table);
}

// http://emonpi.local/feed/data.json?ids=3,6,9,10,11,12&start=1716719857&end=1716719867&interval=10
// this is in timestamp_unix_ms

// [
//     {
//         "feedid": "3",
//         "data": [
//             [
//                 1716719857000,
//                 -5
//             ],
//             [
//                 1716719867000,
//                 -4
//             ]
//         ]
//     },
//     {
//         "feedid": "6",
//         "data": [
//             [
//                 1716719857000,
//                 27
//             ],
//             [
//                 1716719867000,
//                 24
//             ]
//         ]
//     },
//     {
//         "feedid": "9",
//         "data": [
//             [
//                 1716719857000,
//                 63
//             ],
//             [
//                 1716719867000,
//                 62
//             ]
//         ]
//     },
//     {
//         "feedid": "10",
//         "data": [
//             [
//                 1716719857000,
//                 33
//             ],
//             [
//                 1716719867000,
//                 32
//             ]
//         ]
//     },
//     {
//         "feedid": "11",
//         "data": [
//             [
//                 1716719857000,
//                 57
//             ],
//             [
//                 1716719867000,
//                 58
//             ]
//         ]
//     },
//     {
//         "feedid": "12",
//         "data": [
//             [
//                 1716719857000,
//                 9
//             ],
//             [
//                 1716719867000,
//                 8
//             ]
//         ]
//     }
// ]
