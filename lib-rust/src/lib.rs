use ctor::{ctor, dtor};

#[ctor]
fn on_inject() {
    println!("XTND injected.");
    load_modules();
}

fn load_modules() {
    println!("Loading modules.");
}

#[dtor]
unsafe fn on_unload() {
    libc::printf("XTND unloaded.\n\0".as_ptr() as *const i8);
}