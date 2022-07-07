use std::env;

fn print_usage()
{
    println!("Invalid input! Usage: bacon {{x|d}} {{input number}}");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3
    {
        print_usage();
    }
}

