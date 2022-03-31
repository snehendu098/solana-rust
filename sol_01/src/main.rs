// the previous codes are present in the README.md file

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
    println!("{}", res.unwrap_or(100))
}
