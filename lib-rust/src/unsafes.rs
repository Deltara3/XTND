pub fn print(data: &str) {
    unsafe { libc::printf(data.as_ptr() as *const i8); }
}

pub fn println(data: &str) {
    print(data);
    print("\n");
}