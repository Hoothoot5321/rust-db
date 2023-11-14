use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TestObj {
    pub name: String,
    pub age: u8,
}

#[derive(Serialize, Deserialize)]
pub struct Bang {
    pub sheis: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Test {
    pub msg: String,
}
