// the previous codes are present in the README.md file

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
