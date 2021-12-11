use std::env;
use std::process::exit;

mod pkg; use pkg::pkg;
mod spwn; use spwn::spwn;
fn main() {
    let argv: Vec<String> = env::args().collect();
    match argv[1].as_str() {
        "install" | "remove" | "update" => pkg(argv),
        "build" | "b" => spwn(argv),
        _ => { println!("Unknown command."); exit(0);}
    }
}
