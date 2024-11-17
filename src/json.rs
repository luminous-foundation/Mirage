use crate::{file::{self, get_dir_files}, strings::expand_tilda_to_home};
use colored::Colorize;
use serde::{Deserialize, Serialize};

use std::{fs, path::Path};

pub fn get_template(ident: String) -> Template {
    let fles = get_dir_files(
        expand_tilda_to_home("~/.mirage/templates/"));

    for fle in fles {
        let content = file::read_file(&fle.as_str());
        let template = serde_json::from_str::<Template>(&content.as_str());
        
        if template.is_err() {
            continue;
        }

        let mut tmp = template.unwrap();

        if tmp.ident == ident {
            let abs = expand_tilda_to_home(&tmp.path);
            let pth = Path::new(abs.as_str());
            let res = pth.canonicalize();

            if res.is_err() {
                panic!("Unable to find absolute path!");
            }

            let path = String::from(res.unwrap().to_str().unwrap());
            tmp.path = path;

            return tmp;
        }
    }

    return Template::empty();
}

pub fn remove_template(ident: String) {
    println!("{} {}{}", "Deleting ".red(), ident.purple().bold(), "!".red());

    let fles = get_dir_files(
        expand_tilda_to_home("~/.mirage/templates/"));

    for fle in fles {
        let content = file::read_file(&fle.as_str());
        let template = serde_json::from_str::<Template>(&content.as_str());
        
        if template.is_err() {
            continue;
        }

        let tmp = template.unwrap();

        if tmp.ident == ident {
            fs::remove_file(&fle.as_str()).unwrap();
            fs::remove_file(tmp.path).unwrap();
            break;
        }
    }

    println!("{}", "Done!".green());
}

pub fn get_all_templates() -> Vec<Template>{
    let mut ret = Vec::new();

    let fles = get_dir_files(
        expand_tilda_to_home("~/.mirage/templates/"));

    for fle in fles {
        let content = file::read_file(&fle.as_str());
        let template = serde_json::from_str::<Template>(&content.as_str());
        
        if template.is_err() {
            continue;
        }

        let tmp = template.unwrap();
        ret.push(tmp);
    }

    return ret;
}

pub fn list_templates() {
    let tmps  = get_all_templates();

    let len = tmps.len().to_string();
    let header = format!("{}{}{}", 
                    "Templates (".green().bold(),
                    len.as_str(),
                    ")".green().bold());
    println!("{}", header);
    println!("{}", "--------------------------".blue().bold());

    for tmp in tmps {
        println!("{}", tmp.ident.purple().bold());
    }

    println!("{}", "--------------------------".blue().bold());
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Template {
    pub ident: String,
    pub path: String
}

impl Template {
    pub fn empty() -> Template {
        return Template {
            ident: String::new(),
            path: String::new()
        };
    }
}