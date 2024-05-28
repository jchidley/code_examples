//! Serde array of arrays.
//! [](https://stackoverflow.com/questions/59873674/rust-deserialize-a-json-array-into-a-very-simple-custom-table)
//! [](https://doc.rust-lang.org/nomicon/other-reprs.html)
//! [Use of Serde's #[serde(transparent)]](https://stackoverflow.com/questions/78535051/use-of-serdes-serdetransparent)
use serde::Deserialize;
use serde_json::{self, Result};

const BARE_ARRAY_ARRAY_DATA: &str = r#"
[
  ["0,1", "0,2", "0,3"], 
  ["1,1", "1,2", "1,3"]
]
"#;

#[derive(Deserialize, Debug)]
#[serde(transparent)]
pub struct Row {
    pub cells: Vec<String>,
}

#[derive(Deserialize, Debug)]
#[serde(transparent)]
pub struct Table {
    pub rows: Vec<Row>,
}

impl Table {
    pub fn new(data: &str) -> Result<Table> {
        serde_json::from_str(data)
    }
}

fn main() {
    let table: Table = Table::new(BARE_ARRAY_ARRAY_DATA).unwrap();
    assert_eq!(table.rows.len(), 2);
    println!("{:?}", table);
}
