static mut G: i32 = 100;

#[allow(static_mut_refs)]

fn main() {
    let a: i32 = 100;

    let b: &i32 = &a; // does not own any value but a reference --> reference of the value that a has

    let c: &i32 = &100;

    println!("{} {} {}", a, b, c);

    let s1: String = "Hello World!".to_string();
    println!("Address of s1:{:p}", &s1);

    let s2: &String = &s1;

    println!("Address stored in s2:{:p}", s2);

    let ok: bool = true;

    let ok = "Hello World!"; // Done be mistaken that it is dynamic typing

    let ok: &String = &"Hello World".to_string();

    let s3: &str = "Hello World!"; // this is a &str , the means it is also called as string slice

    // references there are two types of references ref and mutable refernce
    let mut a = 100;

    let b = &mut a;

    *b = 200;

    let mut a = 500.343; // here a is diffent variable

    *(&mut a) = 1000.14;

    let a = &mut 300; // what is the type a ? &mut i32
    *(a) = 1000;
    unsafe {
        G = 200;
        println!("G={G}");
    }
}
// reference is a value the let to get the access of another value
// without taking the ownership
// is reference a pointer --> Yes kind of a pointer but not a full fledged pointer.
// ref is a safe typed pointer, rust look after this ref by appling borrowing and lifetime rules.
