// Ownership -> Who(which variable) is the owner of the data
fn main() {
    let a = 100;
    let b = a; // actually it is supposed to be the ownership transfer but it doesn't

    let s1 = "Hello World!".to_string();
    let s2 = s1; // s2 is the owner of the data/value "Hello World!"
}
// Every value/data in Rust has an owner
// There can be only one owner at a time
// When owner goes out of the scope, the value/data is dropped

// To work on ownership of value, rust looks at two aspects
// 1. Is the value/data stored in stack -> If Yes, copy the data, dont do ownership transfer, bca rust treats the copy of data is cheap
// 2. Is the value/data stored in heap -> if Yes , the transfer the ownership, dont copy or duplicate the value/data
// every thing to copy the value or transfer the ownership is purely based on some kind of contracts called as traits --> copy and clone traints
// traits are similar to interfaces in other programming languages
