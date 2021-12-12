extern crate libc;
extern crate ctor;

mod unsafes;
use unsafes::println;

use std::fs::read_dir;

use ctor::{ctor, dtor};

#[ctor]
fn init() {
    println!("XTND injected.");

    println!("Loading modules...");
    // TODO implement this
    let total = load_modules();

    println!("Loaded {} modules.", total);
    println!("XTND initalized.");
}

fn load_modules() -> i32 {
    // TODO are you implementing this yet
    println!("Locating modules...");
    let paths = read_dir("~/.xtnd").unwrap();

    let mut counted_modules: i32 = 0;
    for path in paths {
        println!("Found module: {}", path.unwrap().path().display());
        counted_modules = counted_modules + 1;
    }
    return counted_modules;
}

#[dtor]
fn on_unload() {
    println("XTND unloaded.");
}