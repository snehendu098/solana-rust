## Install Rust

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

## Cargo

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

## String

```
fn main() {
    let str: &str = "Hello World";
    let mut string: String = String::from("Hello World");

    println!("{:?}", string);

    // slicing a string from 0th to 6th index
    let slice = &string[..6];
    println!("{:?}", slice);

    // pushing a char value at the end of the string
    let popped = string.push('1');
    println!("{:?}", popped);

    let replaced = string.replace("Hello", "Bye");
    println!("{}", replaced)
}
```

_String.replace must be stored if you want to mutate with the actual one, use -> string=string.replace("Hello", "Bye)_

## Conditionals

Conditionals are the `if else` statements

```
fn main() {
    let mango = 1;

    if mango % 2 == 0 {
        println!("Even")
    } else if mango % 2 != 0 {
        println!("Odd")
    } else {
        println!("doesn't exist")
    }
}
```

## Loops

### For Loops

```
fn main() {
    for i in 0..5 {
        println!("{}", i);
    }
}
```

### While Loops

```
fn main() {
    println!("Decreasing");
    decreasing();

    println!("Increasing");
    increasing()
}

fn decreasing() {
    let mut i = 0;

    while i > -6 {
        println!("{}", i);
        i -= 1;

        if i == -3 {
            println!("exit");
            break;
        }
    }
}

fn increasing() {
    let mut i = 0;

    while i < 6 {
        println!("{}", i);
        i += 1;

        if i == 3 {
            println!("contnue");
            continue;
        }
    }
}

```

`break` keyword breaks the loop while the `continue` keyword continues to execute the loop

## Matches (Switch statement)

These are mainly `switch` statements in rust

```
fn main() {
    let i = 5;

    match i {
        0 => println!("0"), // ";"" is replaced by ",""
        1 | 2 => println!("1,2"),
        3..=6 => println!("1 - 6"), // 3 to 6 not 5

        _ => println!("deafult"), // no case matches
    }
}
```

## Structs (Classes)

There are 3 parts

1. Declearing struct
2. Declearing Functions in Struct
3. Using the struct

```
// 1. Declearing Struct
struct Bird {
    name: String,
    attack: u16,
    id: u64,
}

// 2. Declearing Functions in Struct
impl Bird {
    fn show_id(&self) {
        println!("{}", self.id);
    }

    fn show_all(&self) {
        println!(
            " Id: {} \n Name: {} \n Attck: {}",
            self.id, self.name, self.attack
        );
    }
}

// 3. Using the struct
fn main() {
    let name = String::from("Bird 1");

    let bird = Bird {
        name, // equivalent to name: name
        attack: 2,
        id: 90,
    };

    bird.show_id();
    bird.show_all()
}
```

4. Declearing Traits

```
// 1. Declearing Struct
struct Bird {
    name: String,
    attack: u16,
    id: u64,
}

// 2. Declearing Functions in Struct
impl Bird {
    fn show_id(&self) {
        println!("{}", self.id);
    }

    fn show_all(&self) {
        println!(
            " Id: {} \n Name: {} \n Attack: {}",
            self.id, self.name, self.attack
        );
    }
}

// 3. Using the struct
fn main() {
    let name = String::from("Bird 1");

    let bird = Bird {
        name, // equivalent to name: name
        attack: 2,
        id: 90,
    };

    bird.show_id();
    bird.show_all();

    // 6. Using traits
    println!("\n {} \n {}", bird.can_fly(), bird.is_animal())
}

// 4. Declearing Traits
trait Animal {
    fn can_fly(&self) -> bool;
    fn is_animal(&self) -> bool {
        true
    }
}

// 5. Implementing traits for bird
impl Animal for Bird {
    fn can_fly(&self) -> bool {
        true
    }
}
```

## Enums

1. Creating an Enum
2. Declearing an Enum
3. using an Enum

```
fn main() {
    // 2. Declearing an Enum
    let a = MyEnum::Color(String::from("#64588"));
    let b = MyEnum::Move(true);
    let c = MyEnum::Position { x: 19, y: -70 };

    // 3. Using an Enum
    if let MyEnum::Move(new) = b {
        println!("Move: {}", new);
    }

    if let MyEnum::Position { x, y } = c {
        println!("Move to: \n X: {}\n Y: {}", x, y);
    }

    if let MyEnum::Color(val) = a {
        println!("Colour the place: {}", val);
    }
}

// 1. Creating an Enum
#[derive(Debug)]
enum MyEnum {
    Move(bool),
    Position { x: i8, y: i8 },
    Color(String),
}
```

### Enums vs structs

Enums are used for defining custom types. Here's an example which shows an convenient usecase of Enums in Structs. There are 2 kinds of IP addresses. One is v4 and v6. In structs, we want to store the IP addresses and their kinds also

```
fn main() {
    let address = String::from("127.0.0.108");

    // 3. Declearing struct with Enum
    let ip = IpAddr {
        kind: IpAddrKind::V4,
        address,
    };

    let ip2 = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("190.2873.189"),
    };

    println!("Type: {:?},\nAddress:{}", ip.kind, ip.address);
    println!("\nType: {:?},\nAddress:{}", ip2.kind, ip2.address);
}

// 1. Creating an Enum
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

// 2. Using the struct
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}
```

## Vectors (dynamic arrays)

In rust, arrays are not dynamic like javascript or python. The arrays need to have a specific number of elements in rust. To solve this problem, rust comes up with Vectors. These are arrays which don't actually have a specific length at first. `array.push()` is not available in Rust.

```
fn main() {
    let mut vec: Vec<i64> = vec![];

    println!("{}", vec.len());

    println!("{:?}", vec);
    vec.push(90);
    vec.push(10);
    vec.push(60);
    println!("{:?}", vec);
    println!("{}", vec.len());
}
```

## Hashmaps

It stores value in key, value pairs. But the values and keys for each case should be of a specific type

```
use std::collections::HashMap; // <- Important

fn main() {
    let mut map = HashMap::new();

    map.insert(0, "hi there");
    map.insert(1, "hi 2");
    println!("{:?}", map);
    match map.get(&0) {
        Some(val) => println!("{}", val),
        _ => println!("Default"),
    }
    match map.get(&2) {
        Some(str) => println!("{}", str),
        _error => println!("Default"), // this is equivalent to `_ => println!("Default"),`
    }
    map.remove(&0);
    println!("{:?}", map);
}
```

## Options

Options are commonly paired with pattern matching to query the presence of a value and take action.

Option returns 2 things:

1. None: If the query is not applicable
2. Some: If the query is applicable and the outcome of it

```
fn divider(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}
```

### Way 1

```
fn main() {
    // let res = divider(0.1, 0.0);
    let res = divider(12.02, 10.60);

    println!("{:?}", res);

    match res {
        Some(val) => println!("{}", val),
        None => println!("Not possible"),
    }
}
```

### Way 2

```
fn main() {
    // let res = divider(0.1, 0.0);
    let res = divider(12.00, 10.);

    println!("{:?}", res);

    match res {
        Some(val) => println!("{}", val),
        None => println!("Not possible"),
    }

    // Unwrap function makes the Some to unwrap the actual value
    println!("{:?}, \nUnwrap: {}", res, res.unwrap());

    // Unwrap for None
    let res2 = divider(293.3, 0.0);
    println!("{:?} \nUnwrapped {}", res2, res2.unwrap());
    // returns: 'main' panicked at 'called `Option::unwrap()` on a `None` value'
}
```

## Result

Results are an upgraded version of Option. In Option, if condition doesn't match, we send `None` which means nothing. In result, when an Error Occurs, we can use custom errors which can even be of custom types.

In option, we can return `Some()` and `None`. In result, we pass `Ok()` and `Err()`

```
#[derive(Debug)]
enum MyError {
    Error1,
}

// While Declearing result, you have to pass a valid value for return and the Error Type within `<>`
fn divider(numerator: i64, denominator: i64) -> Result<i64, MyError> {
    if denominator == 0 {
        Err(MyError::Error1)
    } else {
        Ok(numerator / denominator)
    }
}

fn main() {
    let res = divider(20, 5);

    // .is_ok() and .is_err() is automatically provided by Results in Rust
    if res.is_ok() {

        println!("{}", res.unwrap())

    } else if res.is_err() {

        println!("{:?}", res.unwrap_err())
        // returns Error1 when denominator is 0

    }
}
```

**You can also use `match`**

```
fn main() {
    let res = divider(20, 0);

    match res {
        Ok(data) => println!("{}", data),
        Err(data) => println!("{:?}", data),
    }
}
```

`unwrap_or()` can also be used to return a default value when the result is not of type `Ok()`

```
#[derive(Debug)]
enum MyError {
    Error1,
}

fn divider(numerator: i64, denominator: i64) -> Result<i64, MyError> {
    if numerator % denominator != 0 {
        Err(MyError::Error1)
    } else {
        Ok(numerator / denominator)
    }
}

fn main() {
    let res = divider(34, 7);
    // if condition is not fulfilled, it will return 100
    println!("{}", res.unwrap_or(100))
}
```
