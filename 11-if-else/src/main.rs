fn main() {
    let (age, gender) = (8, 'F');

    if age >= 18 && (gender == 'f' || gender == 'F') {
        // true && false || true
        // false || true
        // true
        println!("She is eligible to maary, As age is {}", age);
    } else if age >= 21 && (gender == 'm' || gender == 'M') {
        println!("He is eligible to maary, As age is {}", age);
    } else {
        println!("Not eligible or invalid gender");
    }

    // variation that if has
    // if can be used as an expression in rust

    let is_eligible = if age >= 18 && (gender == 'f' || gender == 'F') {
        true
    } else if age >= 21 && (gender == 'm' || gender == 'M') {
        true
    } else {
        false
    };
    println!("is eligible:{}", is_eligible);

    // if let

    let mut num = 42;

    if let 42 = num {
        println!("number is 42");
    } else {
        println!("number is not 42");
    }

    let person = (21, 'F'); // Type must have been implemented a Display trait
    println!("age:{} gender:{}", person.0, person.1);
    println!("person:{:?}", person); // debug print

    // or pattern
    if let (age, 'F' | 'f') = person {
        println!("Female , age is {}", age);
    } else {
        println!("Pattern does not match");
    }

    // range pattern
    num = 100;
    // refutable pattern
    if let 42..=50 = num {
        println!("number is 42");
    } else {
        println!("number is not 42");
    }

    // irrefutable pattern

    if let _ = num {
        println!("number is 42");
    } else {
        println!("number is not 42");
    }

    // There is no null or nil in rust
    // let ref_num = &mut 100; // keep 100 in memory and assign that 100 to a ref_num
    // *ref_num = 200;

    // there enum Options
    // Optional variable -> The var can have some(Some) value or No value(None)

    let mut num: Option<i128> = None;
    // Payload  -> 16 bytes
    // Discriminant -> 1 byte

    // let num: i32;

    println!("{:#?}", num);
    num = Some(100);

    if let Some(val) = num {
        println!("num value is :{}", val);
    } else {
        println!("num has no value, that is None");
    }

    println!("num size:{}", std::mem::size_of_val(&num));
    num = None;
    println!("num size:{}", std::mem::size_of_val(&num));
}
