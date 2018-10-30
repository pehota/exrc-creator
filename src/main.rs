// use std::collections::HashMap;
use std::env;
// use std::fs::File;
use std::vec::Vec;

#[derive(Debug, PartialEq, PartialOrd)]
struct Args {
    dst_dir: String,
    languages: Vec<String>,
}

impl Args {
    fn empty() -> Args {
        Args {
            dst_dir: String::from(""),
            languages: vec![],
        }
    }
}

fn main() {
    let args = get_args();
    println!("Args: {:#?}", args);

    if args.languages.is_empty() {
        println!("Please provide languages to use in the .exrc");
        return;
    }
}

fn get_args() -> Args {
    let mut arguments = Args::empty();
    let args: Vec<String> = env::args().collect();
    let mut arg_name: String = "".to_string();

    for arg in args.iter().skip(1) {
        if arg[..1] == "-".to_string() && arg_name.is_empty() {
            arg_name = arg[1..arg.len()].to_string();
        } else if !arg_name.is_empty() {
            if arg_name == "dst".to_string() {
                arguments.dst_dir = arg.to_string();
            } else if arg_name == "languages".to_string() {
                arguments.languages = arg
                    .to_string()
                    .split(',')
                    .map(String::from)
                    .collect::<Vec<String>>();
            }
            arg_name = "".to_string();
        }
    }

    arguments
}
