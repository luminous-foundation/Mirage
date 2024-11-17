use core::panic;
use std::fs;
use std::fs::File;
use std::io::Write;

use crate::strings::expand_tilda_to_home;

pub fn read_file(path: &str) -> String {
    let content = fs::read_to_string(path);

    if content.is_err() {
        panic!("Failed to read file {}!", String::from(path));
    }

    return content.unwrap();
}

pub fn write_to_file(vec: Vec<String>) {
    let file = File::create("./.gitignore");

    if file.is_err() {
        panic!("Failed to create .gitignore!");
    }

    let mut fle = file.unwrap();
    for mut line in vec {
        line.push('\n');
        let wrt = fle.write(line.as_bytes());

        if wrt.is_err() {
            panic!("Failed to write to .gitignore!");
        }
    }
}

pub fn write_to(content: String, fname: String) {
    let file = File::create(fname);
    
    if file.is_err() {
        panic!("Failed to create .gitignore!");
    }

    let mut fle = file.unwrap();
    fle.write(content.as_bytes()).unwrap();
}

pub fn get_dir_files(path: String) -> Vec<String> {
    let mut ret = Vec::new();
    
    let contents = fs::read_dir(path);

    if contents.is_err() {
        panic!("Failed to read file!");
    }

    for fle in contents.unwrap() {
        let unwrapped= fle.unwrap();

        if unwrapped.path().is_file() {
            let path = unwrapped.path();
            let path_str = path.to_str().unwrap();
            
            ret.push(path_str.to_string());
        }
    }

    return ret;
}

pub fn init_app_dir() {
    if !fs::exists(expand_tilda_to_home("~/.mirage/")).unwrap() {
        fs::create_dir(expand_tilda_to_home("~/.mirage/")).unwrap();
    }

    if !fs::exists(expand_tilda_to_home("~/.mirage/templates")).unwrap() {
        fs::create_dir(expand_tilda_to_home("~/.mirage/templates")).unwrap();
    }
}