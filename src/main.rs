use crate::{extract::extract_person, model::Person};

mod extract;
mod model;

fn main() {
    let p = extract_person().unwrap();

    println!("{:?}", p);
}
