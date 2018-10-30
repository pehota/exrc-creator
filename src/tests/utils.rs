#[cfg(test)]
#[test]
fn parse_args() {
    use utils::{parse_args, Args};

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
