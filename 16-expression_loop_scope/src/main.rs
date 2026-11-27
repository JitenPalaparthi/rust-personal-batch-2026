use rand::RngExt;

fn main() {
    let mut num = 1;

    // loop can be an expression

    let (mut even, mut odd, mut sum) = (0, 0, 0);

    let result = loop {
        if num > 27 {
            break (even, odd, sum); // loop as an expression
        }
        if num % 2 == 0 {
            even += 1;
        } else {
            odd += 1;
        }
        sum += num;
        num += 1;
    };

    println!(
        "even count:{} odd count:{} sum:{}",
        result.0, result.1, result.2
    );

    // using a scope as an expression

    let mut rng = rand::rng();
    let is_even_result = {
        let rnd_num = rng.random_range(0..100000);
        if rnd_num % 2 == 0 {
            (rnd_num, true)
        } else {
            (rnd_num, false)
        }
    };

    println!("rand_num:{} is_even:{}", is_even_result.0, is_even_result.1);

    // let (mut i, mut sum) = (1, 0);

    // let mut r = while i * 2 < 10 {
    //     sum += i;
    //     i += -1;
    //     if true {
    //         break sum;
    //     }
    // }; // expression is only to check the conditon of the loop, not to evaluate and give back from the loop
}
