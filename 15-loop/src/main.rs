fn main() {
    let (mut a, mut b, mut num) = (0, 1, 0);
    println!("{a} {b} {num}");

    loop {
        if num > 10 {
            break;
        }

        print!("{} ", a);

        (a, b) = (b, a + b); // no need to use a temp variable, can directly do it

        num += 1;
    }
    println!();
    num = 1;
    let mut sum: i64 = 0;

    loop {
        if num > 10 {
            break;
        }
        sum += num as i64; // type casting
        if num % 2 == 0 {
            num += 1;
            continue; // continue to the next iteration while skipping the current one
        }
        print!("{}", num);
        num += 1;
    }

    println!("\nsum:{sum}");

    println!();
    let (mut i, mut j) = (1, 1);

    loop {
        if i > 5 {
            break;
        }
        j = 1;
        loop {
            if j > 5 {
                break;
            }
            println!("i = {i}-->j = {j}");
            j += 1;
        }
        i += 1;
    }

    println!("Break inner and outer with done");
    let (mut i, mut j, mut done) = (1, 1, false);

    loop {
        if i > 5 || done {
            break;
        }
        j = 1;
        loop {
            if j > 5 {
                done = true;
                break;
            }
            println!("i = {i}-->j = {j}");
            j += 1;
        }
        i += 1;
    }

    println!("Break inner and outer with label");
    let (mut i, mut j) = (1, 1);

    'exit: loop {
        if i > 5 {
            break;
        }
        j = 1;
        loop {
            if j > 5 {
                break 'exit;
            }
            println!("i = {i}-->j = {j}");
            j += 1;
        }
        i += 1;
    }
}
