
#[no_mangle] // prevent rust from making ur function some weird name like [[name here]]
fn init() {
    println!("Hello from another lib o/");
}