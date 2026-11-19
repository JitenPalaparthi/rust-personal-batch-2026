fn main() {
    // let a = 10;
    // let b = a;

    let s1 = "Hello World!".to_string();

    let l = get_len(s1); // the ownership is transffered to the fn, so s1 no longer owns any data
    println!("l={l}");
    // let s2 = s1; // so this statement is invalid

    let s1 = "Hello World!".to_string();

    let l = get_lenr(&s1); // borrow the ownership which is temporarely give the ownership
    println!("l={l}");

    // immutable borrow
    let s2 = &s1; // s1 is the owner but borrowed to s2

    let s3 = &s1; // temporarly gives the ownership to s3

    let s4 = &s1; //

    println!("{} {} {} {} ", s1, *s2, *s3, s4); // no need to say *s2 etc...
}
// move
// ownership transfer
// reference
// lifetime
//
fn get_len(s: String) -> usize {
    return s.len();
}

fn get_lenr(s: &String) -> usize {
    return s.len();
}
