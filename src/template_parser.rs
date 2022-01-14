use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn run() {
    // pub const TEMPLATE:Vec<String> = read_file("template/template.html");
    println!("HEADER {:?}", parse_header());
    println!("BODY {:?}", parse_body());
    println!("FOOTER {:?}", parse_footer());
}

fn read_file(filename: &str) -> Vec<String> {
    let template_file = File::open(filename).expect("unable to open template file");
    let template_file = BufReader::new(template_file);
    let mut template: Vec<String> = Vec::new();
    for line in template_file.lines() {
        let line = line.expect("unable to read file");
        template.push(line.trim().parse().unwrap());
    }
    template
}

pub fn parse_header() -> Vec<String> {
    let template = read_file("template/template.html");
    let start = template
        .iter()
        .position(|r| r == "<!DOCTYPE html>")
        .unwrap_or(0);
    let end = template
        .iter()
        .position(|r| r == "</head>")
        .unwrap_or(template.len());
    template[start..end + 1].to_vec()
}

pub fn parse_body() -> Vec<String> {
    let template = read_file("template/template.html");
    let start = template.iter().position(|r| r == "<body>").unwrap_or(0);
    let end = template
        .iter()
        .position(|r| r == "</body>")
        .unwrap_or(template.len());
    template[start..end + 1].to_vec()
}

pub fn parse_footer() -> Vec<String> {
    let template = read_file("template/template.html");
    let start = template.iter().position(|r| r == "</body>").unwrap_or(0);
    let end = template
        .iter()
        .position(|r| r == "</html>")
        .unwrap_or(template.len());
    template[start + 1..end + 1].to_vec()
}
