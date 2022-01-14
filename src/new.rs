use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

pub fn create(path: &str) {
    //create starter files for new project
    if !Path::new(path).exists() {
        println!("Creating new project: {}", path);
        fs::create_dir_all(path).expect("cannot create project directory");
        fs::create_dir_all(format!("{}/content", path)).expect("cannot create folders");
    } else {
        println!("directory named \"{}\" already exists", path);
    }

    let get_started = include_str!("../template/Get_Started.md");
    let mut get_started_file =
        File::create(format!("{}/content/Get_Started.md", path)).expect("error");
    get_started_file
        .write_all(get_started.as_bytes())
        .expect("error writing Get_Started.md");
}
