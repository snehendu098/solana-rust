fn main() {
    // unsigned integers
    // u8, u16, u32, u64, u128
    let unsigned : u8 = 100;

    // signed integers
    // i8, i16, i32....

    let signed : i32 = -783;

    // float: for decimals
    // f8, f16....
    let float = 0.96;

    println!("unsigned: {}, signed {}, float {}", unsigned, signed, float + 0.96);

    // char - can only be
    let letter = "I am a letter";
    let emoji = "\u{1F600}";
    println!("letter {}, emoji {}", letter, emoji);

    let is_true = false;
    println!("Bool : {is_true}");
}
