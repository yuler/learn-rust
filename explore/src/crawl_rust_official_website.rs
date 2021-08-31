use std::fs;

fn main() {
    let url = "https://www.rust-lang.org";
    let output = "rust.md";

    println!("Fetching url: {}", url);
    let html = reqwest::blocking::get(url).unwrap().text().unwrap();

    println!("Converting html to markdown...");
    let md = html2md::parse_html(&html);

    fs::write(output, md.as_bytes()).unwrap();
    println!("Converted markdown has been saved in {}.", output);
}
