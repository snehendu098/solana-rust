// the previous codes are present in the README.md file

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
