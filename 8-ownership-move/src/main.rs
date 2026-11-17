use std::str::FromStr;

fn main() {
    let mut s1 = "Hello World!".to_string();
    //let l = get_len(s1);
    //println!("len:{}", l);
    println!("s1:{s1}");

    let t = (10, true, "Hello World!", 'C');
    println!("{} {} {}", t.0, t.1, t.2);

    let (a1, b1, c1, _) = t;
    println!("{} {} {}", a1, b1, c1);

    let (l, s1) = get_len(s1);
}

// fn get_len(s: String) -> usize {
//     return s.len();
// }

fn get_len(s: String) -> (usize, String) {
    return (s.len(), s);
}
