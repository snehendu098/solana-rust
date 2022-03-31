// the previous codes are present in the README.md file

fn divider(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}

fn main() {
    // let res = divider(0.1, 0.0);
    let res = divider(12.02, 10.60);

    println!("{:?}", res);

    match res {
        Some(val) => println!("{}", val),
        None => println!("Not possible"),
    }
}
