# Install Rust

### Create a file and write basic hello world

```
fn main() {
    println!("hello world");
}
```

### Rust Build:

In terminal write
`rustc ./<YOUR_FILE_NAME>`

##### You will see:

An .exe file generated in windows
A file with the corresponding name of file in linux

Run that file
`./<NAME_OF_FILE>`

# Cargo

In terminal
`cargo new <FOLDER_NAME>`

**It might take some time**

## Basic Cargo commands to get started

_In `Cargo.toml` file, make sure the name of package is in snake case_

```
cargo build <-- builds your rust file

cargo run <-- builds and runs your file at the same time
```

### Scaler types

```
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
```

### Compound types

Compound type variables are arrays, tuples

#### Array

Arrays in rust can contain one kind of values

```
fn main() {
    let arr = [100, 3, 5, 56 ];

    println!("Array {}, {}", arr[0] + arr[3], arr.len());

    // print structure of array and other objects
    println!("Array {:?}", arr);

    let other_array = [50; 20];
    // prints 20 50s
    println!("Other {:?}", other_array)
}
```
