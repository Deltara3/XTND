#![crate_type = "dylib"]

extern crate libc;
extern crate ctor;
extern crate dirs;
extern crate libloading;

use std::fs::read_dir;
use std::env::consts::OS;

use dirs::home_dir;

use ctor::{ctor, dtor};
use libloading::Library;

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

        let expected_extension = std::ffi::OsStr::new(match OS {
            "windows" => "dll",
            "macos" => "dylib",
            "linux" => "so",
            _ => { panic!("Unsupported operating system."); }
        });

        if module_extension == expected_extension {
            println!("Found module: {:?}", module_path);

            // open up module
            let module_lib = unsafe { Library::new(&*module_path.to_string_lossy()).unwrap() };// what the fuck

            let init: libloading::Symbol<unsafe extern fn() -> &'static str> = unsafe { module_lib.get(b"init").unwrap() };

            unsafe { init() };

            counted_modules += 1;
        }
    }
    return counted_modules;
}

#[dtor]
fn on_unload() {
    unsafe { libc::printf("XTND unloaded.\n\0".as_ptr() as *const i8); }
}