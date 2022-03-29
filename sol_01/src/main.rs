fn main() {
    let arr = [100, 3, 5, 56 ];

    println!("Array {}, {}", arr[0] + arr[3], arr.len());

    // print structure of array and other objects
    println!("Array {:?}", arr);

    let other_array = [50; 20];
    // prints 20 50s
    println!("Other {:?}", other_array)
}
