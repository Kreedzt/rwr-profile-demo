use crate::{extract::extract_person, file::save_to_file, model::Person, save::save_person};

mod extract;
mod file;
mod model;
mod save;

fn main() {
    let p = extract_person().unwrap();

    println!("{:?}", p);

    println!("================================================");
    println!("=========extract person completed===============");
    println!("================================================");

    let xml_res = save_person(&p).unwrap();

    println!("{}", xml_res);

    println!("===========================================");
    println!("==========xml save completed===============");
    println!("===========================================");

    save_to_file(&xml_res);

    println!("{}", serde_json::to_string(&p).unwrap());

    println!("===========================================");
    println!("==========json send completed==============");
    println!("===========================================");
}
