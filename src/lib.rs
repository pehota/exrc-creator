use std::error::Error;
use std::vec::Vec;
// use std::fs::File;

#[derive(Debug, PartialEq)]
pub struct Config {
    pub dst_dir: String,
    pub languages: Vec<String>,
}

impl Config {
    pub fn empty() -> Config {
        Config {
            dst_dir: String::from(""),
            languages: vec![],
        }
    }
    pub fn new(args: Vec<String>) -> Result<Config, &'static str> {
        let mut arguments = Config::empty();
        let mut arg_name: Option<String> = None;

        for arg in args.iter() {
            if arg.starts_with("-") && arg_name.is_none() {
                arg_name = Some(arg.to_string());
            } else if let Some(argument_name) = arg_name {
                if vec!["--dst".to_string(), "-d".to_string()].contains(&argument_name) {
                    arguments.dst_dir = arg.to_string();
                } else if vec!["--languages".to_string(), "-l".to_string()].contains(&argument_name)
                {
                    arguments.languages = arg
                        .to_string()
                        .split(",")
                        .map(String::from)
                        .collect::<Vec<String>>();
                }
                arg_name = None;
            }
        }

        if arguments.languages.is_empty() {
            return Err("No languages provided");
        }

        Ok(arguments)
    }
}

pub fn run(_config: Config) -> Result<(), Box<Error>> {
    Ok(())
}

mod tests {
    #[cfg(test)]
    #[test]
    fn parse_args() {
        use super::*;

        assert_eq!(
            Config::new(
                String::from("./exrc-creator --dst root --languages elm,rust")
                    .split(' ')
                    .map(String::from)
                    .collect::<Vec<String>>()
            ),
            Ok(Config {
                dst_dir: String::from("root"),
                languages: vec!["elm".to_string(), "rust".to_string()]
            })
        );

        // short argument names
        assert_eq!(
            Config::new(
                String::from("./exrc-creator -d root -l elm,rust")
                    .split(' ')
                    .map(String::from)
                    .collect::<Vec<String>>()
            ),
            Ok(Config {
                dst_dir: String::from("root"),
                languages: vec!["elm".to_string(), "rust".to_string()]
            })
        );
    }
}
