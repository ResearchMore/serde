#[macro_use]
extern crate serde_derive;

#[derive(Serialize)]
struct S {
    #[serde(rename(serialize = "x"))]
    #[serde(rename = "y")]
    x: (),
}

fn main() {}
