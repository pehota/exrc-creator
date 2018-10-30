use std::vec::Vec;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Args {
    pub dst_dir: String,
    pub languages: Vec<String>,
}

impl Args {
    fn empty() -> Args {
        Args {
            dst_dir: String::from(""),
            languages: vec![],
        }
    }
}

pub fn parse_args(args: Vec<String>) -> Args {
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
