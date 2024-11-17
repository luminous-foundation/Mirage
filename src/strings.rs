use std::env;

pub fn split_string_on_char(str: &str) -> Vec<String> {
    let mut ret = Vec::new();
    
    let mut s = String::new();
    for chr in str.chars() {
        match chr {
            ',' => {
                ret.push(s);
                s = String::new();
            }
            _ => {
                s.push(chr);
            }
        }
    }

    ret.push(s);

    return ret;
}

pub fn expand_tilda_to_home(str: &str) -> String {
    let mut ret = String::new();
    let uname = env::var("USER").unwrap();

    for chr in str.chars() {
        match chr {
            '~' => {
                let mut hm = String::from("/home/");
                hm = format!("{}{}", hm, uname);

                ret = format!("{}{}", ret, hm);
            }
            _ => {
                ret.push(chr);
            }
        }
    }

    return ret;
}