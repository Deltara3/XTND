pub fn println(data: &str) {
    unsafe { libc::printf(format!("{}\n\0", data).as_ptr() as *const i8); }
}