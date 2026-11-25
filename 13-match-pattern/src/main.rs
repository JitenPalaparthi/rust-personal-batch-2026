fn main() {
    // basic match patterns

    let day: u8 = 4;

    match day {
        1 => println!("Sunday"), // Arms
        2 => println!("Monday"),
        3 => println!("Tuesday"),
        4 => println!("Wednesday"),
        5 => println!("Thursday"),
        6 => println!("Friday"),
        7 => println!("Saturday"),
        _ => println!("Noday"),
    }

    match day {
        1 | 7 => println!("Weekend (Sun/Sat day"),
        2..=6 => println!("Working day, Mon-Friday"),
        _ => println!("Noday"),
    }
    // match patterns has arms
    // how the arms are picked , they are picked based on the patten

    let direction = Direction::East;

    match direction {
        Direction::North => println!("North"),
        Direction::South => println!("South"),
        Direction::East => println!("East"),
        Direction::West => println!("West"),
        //_ => println!("None direction"),
    }

    let msg = Message::Move { x: 10, y: 20 };

    apply_message_with_if_let(&msg);
    apply_message_with_match_pattern(&msg);

    let num: Option<i32> = Some(100);

    if let Some(v) = num {
        println!("valueL:{}", v);
    } else {
        println!("None");
    }

    match num {
        Some(v) => println!("valueL:{}", v),
        Noe => println!("None"),
    }

    let num = 10;

    match num {
        n if n > 0 && n % 2 == 0 => println!("{} is positive even number", n),
        n if n < 0 && n % 2 == 0 => println!("{} is negative even number", n),
        0 => println!("is zero"),
        _ => println!("{} is positive or negative odd number", num),
    }
}

fn apply_message_with_if_let(msg: &Message) {
    if let Message::Quit = msg {
        println!("Quit");
    } else if let Message::Move { x, y } = msg {
        println!("Move X:{x} Y:{y}")
    } else if let Message::Write(s) = msg {
        println!("Write:{s}");
    } else if let Message::ChangeColour(r, g, b) = msg {
        println!("ChangeColour: R:{} G:{} B:{}", r, g, b);
    } else if let Message::Some { a, b, c, d } = msg {
        println!("Some: a:{} b:{} b:{} d:{}", a, b, c, d);
    } else {
        println!("Unklnown Message");
    }
}

fn apply_message_with_match_pattern(msg: &Message) {
    match msg {
        Message::Quit => {
            println!("Quit");
        }
        Message::Move { x, y } => {
            println!("Move X:{x} Y:{y}")
        }
        Message::Write(s) => {
            println!("Write:{s}");
        }
        Message::ChangeColour(r, g, b) => {
            println!("ChangeColour: R:{} G:{} B:{}", r, g, b);
        }
        Message::Some { a, b, c, d } => {
            println!("Some: a:{} b:{} b:{} d:{}", a, b, c, d);
        }
    }
}
// enum is a type that can have one or more/possible variants
#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug)]
enum Message {
    Quit,                                    // 1 byte
    Move { x: i32, y: i32 },                 // 4 + 4
    Write(String),                           // 24 bytes
    ChangeColour(u8, u8, u8),                // 1 + 1 + 1
    Some { a: i64, b: i64, c: i64, d: i64 }, // 8 + 8 + 8 + 8
} // Discriminant --> 1 byte 32 + 1 -> 33 + 7 -- 40 bytes
