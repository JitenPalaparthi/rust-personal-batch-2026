fn main() {
    println!("Hello, world!");
    let num1: i8 = 127;
    let num2: i16 = 3213;
    let num3: i32 = 1214312;
    let num4: i64 = -343242342334232434;
    let num5: i128 = -43243242342344324234234234234;

    let num6: u8 = 255;

    let num7: u16 = 15999;
    let num8: u32 = 65001;
    let num9: u64 = 9999999999999999999;
    let num10: u128 = 99999999999999999999999999999999999999;

    let num11: isize = 21312323; // size of the variable is determined by the processor
    let num12: usize = 5345345435;

    // IEEE 754 --> Sign, Exponent, Mantissa
    // 1 but is dedicated for sign
    // 8 bits are for exponent , bcz of 8 bits , the exponent can be 2/10 to the power of -127 to +127
    // 23 bits are design for binary digits
    // computers use base 2 not base 10, 10 is used since easy to calculate
    let f1: f32 = 1.25; // 1.25 * 10 power of 0
    let f2: f32 = 12.5; // 1.25 * 10 power of 1
    let f3: f32 = 125.0; //1.25 * 10 power of 2

    // Value = (-1)S X (1.M) (base2) X 2 2 power of E-127

    let f4: f64 = 123232.123123; // 1+11+52 = 64

    let ok: bool = true;

    let name: &str = "Hello World!"; // This is called as &str a borrow to the string literal

    let c1 = 'A';

    let heart1: char = '\u{2764}';

    let heart2 = '❤';

    let num13 = 0b101010;
    let num14 = 0x1ffe;

    println!(
        "{num1} {num2} {num3} {num4} {num5} {num6} {num7} {num8} {num9} {num10} {num11} {num12} {f1} {f2} {f3} {f4} {ok} {name}"
    );

    println!(
        "{} {} {} bin:{:b} xex:{:x} dec:{} bin:{:b} xex:{:x} dec:{}",
        c1, heart1, heart2, num13, num13, num13, num14, num14, num14
    );
}
