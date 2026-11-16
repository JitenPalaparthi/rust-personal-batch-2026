// Ownership -> Who(which variable) is the owner of the data
fn main() {
    let a = 100;
    let b = a; // actually it is supposed to be the ownership transfer but it doesn't

    // There is a value 100 the owner is a
    // There is another value 100 for which the owner is b

    let s1 = "Hello World!".to_string();
    let s2 = s1; // s2 is the owner of the data/value "Hello World!"

    let s3 = "Hello World! How are you doing? We are learning Rust Programming, Do you think it is useful?".to_string();
    let s4 = "HW!".to_string();

    let s5 = "".to_string();

    // 24 bytes is the size of String Header or String Structure or String Metadata
    let str1 = "Hello World!";
}
// Every value/data in Rust has an owner
// There can be only one owner at a time
// When owner goes out of the scope, the value/data is dropped

// To work on ownership of value, rust looks at two aspects
// 1. Is the value/data stored in stack -> If Yes, copy the data, dont do ownership transfer, bca rust treats the copy of data is cheap
// 2. Is the value/data stored in heap -> if Yes , the transfer the ownership, dont copy or duplicate the value/data
// every thing to copy the value or transfer the ownership is purely based on some kind of contracts called as traits --> copy and clone traints
// traits are similar to interfaces in other programming languages

// Rust follows a rule for ownership transfer

// If the data is in stack --> It copies the data
// If the data is in heap --> The ownership is transffered to the new variable, generally does not copy the data
