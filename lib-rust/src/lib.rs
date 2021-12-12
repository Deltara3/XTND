use ctor::{ctor, dtor};

use crate::unsafe::print;

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
}

#[dtor]
fn on_unload() {
    printf("XTND unloaded.");
}