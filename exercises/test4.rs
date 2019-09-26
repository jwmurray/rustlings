// test4.rs
// This test covers the sections:
// - Modules
// - Macros

// Write a macro that passes the test! No hints this time, you can do it!
#[macro_export]
macro_rules! my_macro {
    () => {
        println!("No arguments");
    };
    ($x:expr) => {
        String::from("Hello ") + $x
    };
}

fn main() {
    if my_macro!("world!") != "Hello world!" {
        panic!("Oh no! Wrong output!");
    }
}
