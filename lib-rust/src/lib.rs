mod unsafes;
use unsafes::{print, println};

use std::fs::read_dir();

use ctor::{ctor, dtor};

#[ctor]
fn init() {
    println("XTND injected.");
    
    println("Loading modules...");
    // TODO implement this
    load_modules();

    println("XTND loaded.");
}

fn load_modules() {
    // TODO are you implementing this yet
    println("Discovering modules...");
    let paths = fs::read_dir("~/.xtnd").unwrap();

    for path in paths {
        print("Dicovered plugin ");
        print(path.unwrap().path().display());
        print("\n");
    }
}

#[dtor]
fn on_unload() {
    println("XTND unloaded.");
}