fn main() {
    println!("Hello, world!");
    // println! is a macro
    let world = "Earth";
    let age: i64 = 4500_000_000;
    println!(
        "Hello {}, How are you doing!\n how old are you? isnt it {} old?",
        world, age
    );
}

// macros are preprocessors, they generate rust code during compilation which is also called as
// _ meta programming
// What is debug build and release build
// debug build contains some code or instrumentation to debug the application
