use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
            panic!("not enough arguments");
    }
    if args.len() > 2 {
            panic!("too much arguments");
    }

    let contents = fs::read_to_string(&args[1]).expect("Something wrong");

    println!("{}", contents);
}
