use std::env;
use std::process::exit;

mod pkg; use pkg::pkg;
mod spwn; use spwn::spwn;
fn main() {
    let argv: Vec<String> = env::args().collect();
    if argv.len() == 1 { println!("Please provide a command"); exit(1); }

    match argv[1].as_str() {
        "install" | "remove" | "update" => pkg(argv),
        _ => spwn(argv)
    }
}
