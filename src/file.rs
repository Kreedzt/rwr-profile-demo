use std::io::Write;

pub fn save_to_file(data: &str) {
    let mut file = std::fs::File::create("res.person").unwrap();
    file.write_all(data.as_bytes());
}
