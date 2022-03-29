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

## Scaler types

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

## Compound types

Compound type variables are arrays, tuples

### Array

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

### Tuples

Tuple can contain multiple types of values

```
fn main() {
    let tuple1: (u8, bool, f32) = (190, true, 2.5);

    let tuple2 = ("hello boy", 78, 97);

    // print values
    println!("1: {}, 2: {}, 3: {}", tuple2.0, tuple2.1, tuple2.2);

    // destructuring
    let (a, b, c) = tuple1;

    println!("1: {}, 2: {}, 3: {}", a, b, c);
}

```

## Function

```
fn main() {
    println!("Even number checker: {}", is_even(42)); // returns true
}

// this is a public function
pub fn is_even(num: u8) -> bool {
    let value = num % 2;
    value == 0 // <- this is a return value so it doesn't have a ";"
}
```

## Mutability

This topic is more of rust specific. In most languages, you can't just mutate(change) the value of a variable. But in rust you can do this by using `mut` keyword

```
fn main() {
    let mut num = 3; // this mut keyword allows the next line to add value to it
    num += 6;

    println!("Number : {num}")
}
```

## Array Slice

Slicing an array means reoving a certain element from the array.

```
fn main() {
    let arr = [1, 2, 4, 5, 6, 7];
    let slice = &arr[1..4]; // starts from the 2nd element and prints till 4th one

    println!("Slice: {:?}", slice) // Slice: [2, 4, 5]
}
```

_More of slicing_

```
fn main() {
    let arr = [1, 2, 4, 5, 6, 7];
    let slice = &arr[1..4]; // starts from the 2nd element and prints till 4th one

    println!("Slice: {:?}", slice); // Slice: [2, 4, 5]

    slicer(arr, slice);
}

fn slicer(arr: [u8; 6], slice: &[u8]) {
    println!("Array {:?}", { arr });
    println!("Slice: {:?}", { slice });
    println!("{} {}", slice[0], slice[1])
}
```
