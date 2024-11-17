use std::env;

use file::init_app_dir;

mod combine;
mod file;
mod strings;
mod json;
mod create;

fn main() {
    let mut args: Vec<_> = env::args().collect();
    args = args[1..].to_vec();

    init_app_dir();

    for (i, _) in args.to_vec().iter().enumerate() {
        match args[i].as_str() {
            "combine" => combine::combine(args[i + 1..].to_vec()),
            "create" => create::create(args[i + 1..].to_vec()[0].clone()),
            "init" => combine::combine(args[i + 1..].to_vec()),
            "del" | 
            "remove" | 
            "delete" | 
            "rem" => json::remove_template(args[i + 1..].to_vec()[0].clone()),
            "list" => json::list_templates(),
            _ => continue
        }
    }
}