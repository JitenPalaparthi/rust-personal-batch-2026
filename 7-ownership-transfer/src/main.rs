static mut G: i32 = 100;
static PI: f32 = 3.14;
static PI1: f32 = 3.14;

#[allow(static_mut_refs)]

fn main() {
    // &str  -> Data Segment
    // String -> Heap

    let mut s1 = "Hello World!".to_string();
    s1.push_str(" How are you doing?");

    let mut s2 = s1; // The ownership is transffered from s1 to s2
    println!("{s2}");
    // once the owner goes out of the scope, the value is dropped

    //println!("{s1}"); // s1 is no longer a valid variable, it would be dropped,
    //

    // let a = 100;

    // let b = a; // There is no move, just a copy

    let b1 = Box::new(4343); // Box is a smart pointer, it contains a pointer on heap. The value 4343 is allocated on heap
    let mut b2 = b1;

    *b2 = 4353;

    println!("address of s2:{:p}", &s2);
    println!("address s2 has:{:p}", s2.as_ptr());
    println!("address b2 has:{:p} ", b2);
    println!("address of b2:{:p}", &b2);
    let a = 100;
    println!("address of a:{:p}", &a);

    unsafe {
        println!("Address of G:{:p}", &G);
    }
    println!("Address of PI:{:p}", &PI);
    println!("Address of PI1:{:p}", &PI1);

    //b2:0x104bd93f0 b2:0x16ba1a490
    // println!("{}", b1);
}

// ownership
// move
// borrow

// There is no nil or Null in rust.

// address of s2:   0x16b20a300
// address s2 has:  0x105361400
// address b2 has:  0x1053613f0
// address of b2:   0x16b20a340
// address of a:    0x16b20a3f4
// Address of G:    0x104c40220
// Address of PI:   0x104c2b93c
// Address of PI1:  0x104c2b940

// address of s2:   0x16dd86ac0
// address s2 has:  0x1026fd400
// address b2 has:  0x1026fd3f0
// address of b2:   0x16dd86ad8
// address of a:    0x16dd86ae4
// Address of G:    0x1020c0220
// Address of PI:   0x1020af2f0
// Address of PI1:  0x1020af2f4
