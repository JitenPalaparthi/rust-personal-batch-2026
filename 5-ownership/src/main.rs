fn main() {
    {
        let a = 100;
        let mut b = a; // Here a copy trait works, so that the value is copied and assigned to the new variable
        b = 200;
        println!("a:{a} b:{b}")
    } // a an b are dropped
    // println!("{a}"); // no longer a's value is availabile in the memory, yes same stack scope

    let mut s1 = "Hello World!".to_string();
    let mut s2 = s1; // This is ownership transfer, why not copy? bcz heap allocations are costly so normally in rust no copy trait is implemented on heap allocated types

    //s1.push_str(" How are you doing!"); // Cannot mutate the value using s1
    // println!("s1:{s1} s2:{s2}");
}

// Rust Application
// String (Vec)
// Rust Global Allocator
// glibc malloc()
// Linux Kernel
// mmap()/brk()
// Virtual Memory
// Physical Memory
