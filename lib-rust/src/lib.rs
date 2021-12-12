mod unsafes;
use unsafes::{print, println};

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

}

#[dtor]
fn on_unload() {
    println("XTND unloaded.");
}