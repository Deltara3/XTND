extern crate libc;
extern crate ctor;
extern crate dirs;
extern crate libloading;

mod unsafes;
use unsafes::println;

use std::fs::read_dir;
use dirs::home_dir;

use ctor::{ctor, dtor};
use libloading::{Library, Symbol};

#[ctor]
fn init() {
    println!("XTND injected.");

    println!("Loading modules...");
    // TODO implement this
    let total = load_modules();

    println!("{} modules loaded.", total);
    println!("XTND initalized.");
}

fn load_modules() -> i32 {
    // TODO are you implementing this yet
    let module_list = read_dir(home_dir().unwrap().join(".xtnd")).unwrap();

    // Potential cringe.
    let mut counted_modules: i32 = 0;
    for module in module_list {
        let unwrapped_module = module.unwrap();
        let module_path = unwrapped_module.path();
        let module_extension = module_path.extension().unwrap();

        #[cfg(target_os = "windows")]
        let expected_extension = std::ffi::OsStr::new("dll");
        #[cfg(target_os = "macos")]
        let expected_extension = std::ffi::OsStr::new("dylib");
        #[cfg(target_os = "linux")]
        let expected_extension = std::ffi::OsStr::new("so");

        if module_extension == expected_extension {
            println!("Found module: {:?}", module_path);
            counted_modules += 1;
        }
    }
    return counted_modules;
}

#[dtor]
fn on_unload() {
    println("XTND unloaded.");
}