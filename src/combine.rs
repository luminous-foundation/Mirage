use colored::Colorize;

use crate::{file::{self, write_to_file}, json::get_template, strings::split_string_on_char};

pub fn combine(args: Vec<String>) {
    println!("{} {}{}", "Initializing".green(), "gitignore".purple().bold(), "!".green());

    let mut buff = String::new();
    
    for arg in args {
        buff.push_str(&arg);
    }

    let mut contents: Vec<String> = Vec::new();

    for str in split_string_on_char(&buff) {
        let tmp = get_template(str);
        let content = file::read_file(&tmp.path);

        contents.push(content);
    }

    write_to_file(contents);

    println!("{}", "Done!".green());
}