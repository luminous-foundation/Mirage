use crate::{file::{read_file, write_to}, json::Template, strings::expand_tilda_to_home};
use colored::*;

pub fn create(name: String) {

    println!("{} {}", "Creating template".green(), name.bold().purple());

    let content = read_file("./.gitignore");

    let mut temp: Template = Template::empty();
    temp.ident = name.clone();
    temp.path = 
    expand_tilda_to_home("~/.mirage/templates/") 
    + name.as_str() 
    + ".txt";

    write_to(content, temp.path.clone());

    let json_r = serde_json::to_string(&temp);

    if json_r.is_err() {
        panic!("Failed to convert template to json!");
    }

    let json = json_r.unwrap();
    let j_path = 
    expand_tilda_to_home("~/.mirage/templates/") 
    + name.as_str() 
    + ".json";
    write_to(json, j_path);

    println!("{}", "Done!".green());
}