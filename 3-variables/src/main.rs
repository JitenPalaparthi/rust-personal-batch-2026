static global: i32 = 100; // this is allocated on Data Segment --> It is a staic variable
const PI: f32 = 3.14; // This is allocated on Data Segment --> It is a constant

fn main() {
    // each function has its own stack frame
    let a = 10; // starts scope of a , lifetime of a
    {
        let b = 20; // starts scope of b, lifetime of b
        let s1 = "Hello World".to_string(); // s1 is allocated on stack , but the orignal data is allocated on heap
        {
            let c: i64 = 30; // stats scope of c, lifetime of c
            let d: u8 = 100; // starts the scope of d, lifetime of d
        } // ends scope of c and d
    } // ends scope of b
    //println!("{s1}");
    // |
    // |
    // |
    // |
} // end of scope of a
// different kinds of memories
// Process memory
// Text/Code Segment
// Data Segment --> Static , Global and also Constants,string literals are stored DS
// Stack Memory --> Process contains 1- more threads. Each thread contain 2mb(depends of OS and settings but generally) of Stack Memory
//          all functions, local variables, function parameters, etc stored in stack memory
//          Stack memory works based on stack pointer
//          Stack pointer keeps on moving by using push or pop operation based on the scope
// Heap Memory -> Can claim as much memory as the program wants(unlike stack 2mb limit), can extend the life of the data , beyond the stack frame where it is created, we carete Heap memory

// When you compile, the compiler, the assembler and linker writes some additional code which is called ass
// instrumentation which is used to give addresses(yes just offsets after compilation) and also writes the code to move stack pointer

// in rust irrespective or stack or heap, the deallocation is based on lifetime of the data/variable
// when the lifetime ends, automatically the data/variable is dropped from the memory
// if stack , stack pointer is moved, if on heap deallocation happen
// what about data segment and code segment, they are unchanged like any other programming langauge
//      memory exists as long as the application run
