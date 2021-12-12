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
    load_modules();

    println!("XTND loaded.");
}

fn load_modules() {
    // TODO are you implementing this yet
    println!("Discovering modules...");
    let paths = read_dir("~/.xtnd").unwrap();

    for path in paths {
        println!("Dicovered plugin {}", path.unwrap().path().display());
    }
}

#[dtor]
fn on_unload() {
    println("XTND unloaded.");
}