fn main() {
    let s1 = "Hello World! 💕".to_string(); // collection

    println!("length:{}", s1.len());
    for b in s1.as_bytes() {
        println!("byte:{} char:{} ", b, *b as char); // in a loop that iterates every byte, take the byte casted to char
    }

    println!("");
    for c in s1.chars() {
        println!(" char:{} ", c); // in a loop that iterates every byte, take the byte casted to char
    }

    // while

    println!("normal while loop");
    let mut num = 1;

    while num <= 10 {
        print!(" {} ", num);
        num += 1;
    }
    println!();
}

// Type conversion
// as a is a type casting
