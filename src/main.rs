// use std::collections::HashMap;
use std::env;
// use std::fs::File;
use std::vec::Vec;

#[derive(Debug, PartialEq, PartialOrd)]
struct Args {
    script: String,
    dst_dir: String,
    languages: Vec<String>,
}

impl Args {
    fn empty() -> Args {
        Args {
            script: "".to_string(),
            dst_dir: String::from(""),
            languages: vec![],
        }
    }

    // fn is_empty(&self) -> bool {
    // let empty = Args::empty();
    // self.script == empty.script
    // && self.dst_dir == empty.dst_dir
    // && self.languages == empty.languages
    // }
}

fn main() {
    let args = get_args();
    println!("Args: {:#?}", args);

    if args.languages.is_empty() {
        println!("Please provide languages to use in the .exrc");
        return;
    }
}

fn resolve_arg_name(arg: String) -> Option<String> {
    None
}

fn get_args() -> Args {
    let mut arguments = Args::empty();
    let args: Vec<String> = env::args().collect();
    let mut arg_name: String = "".to_string();

    for arg in args.iter().skip(1) {
        println!("current arg: {}", arg);
        if arg[..1] == "-".to_string() && arg_name.is_empty() {
            arg_name = arg[1..arg.len()].to_string();
        } else if !arg_name.is_empty() {
            if arg_name == "dst".to_string() {
                arguments.dst_dir = arg.to_string();
            } else if arg_name == "languages".to_string() {
                // arguments.languages = arg
                // .to_string()
                // .split(',')
                // .collect::<Vec<&str>>()
                // .into_iter()
                // .map(|str| str.to_string())
                // .to_vec();

                println!(
                    "split: {:?}",
                    arg.to_string().split(',').collect::<Vec<&str>>()
                );
            }
            arg_name = "".to_string();
        }
    }

    arguments
}
