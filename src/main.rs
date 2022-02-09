use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("not enough arguments");
    }
    if args.len() > 2 {
        panic!("too much arguments");
    }
    let contents = fs::read_to_string(&args[1]).expect("Something wrong");

    let mut infected: Vec<&str> = Vec::new();
    for line in contents.lines() {
        if line.contains("moved to") {
            //println!("{}", &line);
            infected.push(&line);
        }
    }
    if infected.len() == 0 {
        println!("There are no infected items");
        process::exit(0);
    } else {
        for inf in infected {
            println!("{}", inf);
        }
        process::exit(2);
    }
}
