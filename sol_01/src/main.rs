// the previous codes are present in the README.md file

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
