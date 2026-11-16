static mut G: i32 = 100; // allocated on datasegment

fn main() {
    let mut a1: i32 = 100; // this is allocated on stack
    let mut b1: Box<i32> = Box::new(100); // since box it is allocarted on heap

    let mut a2: i128 = 1432432;

    let mut b2: Box<i128> = Box::new(1432432); // Box contain a pointer to the memory in Heap

    let mut s1: &str = "Hello Rust Engineers!"; // &str the data is stored in .rodata which is part of data segment.

    let mut s2: &str = "你好，世界！ ❤️ Hello, world!"; // &str is a collection of bytes, form as chars.

    let mut s3: &str = "你好，世界！ ❤️ Hello, world!"; // &str is a collection of bytes, form as chars.

    let mut s4: &str = "你好，世界！ ❤️ Hello, world!"; // &str is a collection of bytes, form as chars.

    let mut s5: &str = "你好，世界！ ❤️ Hello, world!"; // &str is a collection of bytes, form as chars.

    s2 = "Hello World ❤️!";

    let mut s6: String = "Hello Rust Engineers!".to_string(); // It is allocated on Heap

    const PI: f32 = 3.14; // This is allocated on .rodata which is datasegment

    println!("Len of s1:{}", s1.len());
    println!("Len of s2:{}", s2.len());

    unsafe {
        G = 200;
    }
}
