use std::mem;

use crate::Message::ChangeColour;

fn main() {
    let direction: Direction = Direction::North;
    println!("{:?}", direction);

    let num: Option<i32> = Some(100); // Some(v),None

    // Message

    let mut msg: Message = Message::Quit;

    println!("msg size:{}", std::mem::size_of_val(&msg));
    apply_message(&msg);

    msg = Message::Write("Hello World!".to_string());

    apply_message(&msg);
    println!("msg size:{}", std::mem::size_of_val(&msg));

    msg = Message::Move { x: 10, y: 20 };
    apply_message(&msg);
    println!("msg size:{}", std::mem::size_of_val(&msg));

    msg = Message::ChangeColour(12, 84, 42);
    apply_message(&msg);
    println!("msg size:{}", std::mem::size_of_val(&msg));
    msg = Message::Some {
        a: 10,
        b: 20,
        c: 40,
        d: 59,
    };
    apply_message(&msg);
    println!("msg size:{}", std::mem::size_of_val(&msg));
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

fn apply_message(msg: &Message) {
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

// String --> Len -> 8 bytes (isize), Cap -> 8 bytes (isize), Ptr -> 8 Bytes(isize)
// isize = MAX bytes , 1 byte some where it is ununsed
