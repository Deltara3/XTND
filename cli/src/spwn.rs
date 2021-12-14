use std::env::consts::OS;
use std::env::set_var;

use std::process::Command;

//const OS: &str = "macos"; // debugging purposes

pub fn spwn(mut argv: Vec<String>) {
    // TODO spawn spwn process and attach prvw
    // cringe code
    argv.remove(0);
    let mut spwn_out = match OS {
        "windows" => {
            println!("\u{001b}[31m[ERR] Windows isn't supported yet.");
            std::process::exit(1);
            #[allow(unreachable_code)]
            Command::new("spwn").spawn().expect("failed to run spwn") // RUST PLEASE STFU
        },

        "macos" => {
            // TODO add correct path
            println!("\u{001b}[33m[WARN] MacOS support is experimental, use at your own risk.");
            Command::new("spwn")
                .env("DYLD_INSERT_LIBRARIES", "/opt/XTND/XTND.so")
                .env("DYLD_FORCE_FLAT_NAMESPACE", "1")
                .args(argv)
                .spawn()
                .expect("failed to run spwn")
        },

        "linux" => {
            Command::new("spwn")
                .env("LD_PRELOAD", "/opt/XTND/XTND.so")
                .args(argv)
                .spawn()
                .expect("failed to run spwn")
        },
        _ => unimplemented!(),
    };

    #[allow(unused_must_use)]
    spwn_out.wait();

}