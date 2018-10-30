mod tests;
mod utils;
// use std::fs::File;

fn main() {
    use std::env;
    use utils;

    let args = utils::parse_args(env::args().collect());
    println!("Args: {:#?}", args);

    if args.languages.is_empty() {
        println!("Please provide languages to use in the .exrc");
        return;
    }
}
