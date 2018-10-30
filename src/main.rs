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
    let args = parse_args(env::args().collect());
    println!("Args: {:#?}", args);

    if args.languages.is_empty() {
        println!("Please provide languages to use in the .exrc");
        return;
    }
}

fn parse_args(args: Vec<String>) -> Args {
    let mut arguments = Args::empty();
    let mut arg_name: String = "".to_string();

    for arg in args.iter() {
        if arg[..2] == "--".to_string() && arg_name.is_empty() {
            arg_name = arg[2..arg.len()].to_string();
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

#[cfg(test)]
mod tests {
    #[test]
    fn parse_args() {
        use super::parse_args;
        use super::Args;

        assert_eq!(
            parse_args(
                String::from("./exrc-creator --dst root --languages elm,rust")
                    .split(' ')
                    .map(String::from)
                    .collect::<Vec<String>>()
            ),
            Args {
                dst_dir: String::from("root"),
                languages: vec!["elm".to_string(), "rust".to_string()]
            }
        );
    }
}
