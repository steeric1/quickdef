use std::env;

fn main() {
    for arg in env::args().skip(1) {
        let body = reqwest::blocking::get(format!(
            "https://api.dictionaryapi.dev/api/v2/entries/en/{}",
            arg
        ))
        .unwrap()
        .text()
        .unwrap();

        println!("{}", body);
    }
}
