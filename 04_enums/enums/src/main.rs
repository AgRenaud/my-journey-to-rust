// Multiple way to define V4/V6 Ip Address
// 1st
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}
#[derive(Debug)]
struct IpAddr1 {
    kind: IpAddrKind,
    address: String,
}

// 2nd
#[derive(Debug)]
enum IpAddr2 {
    V4(String),
    V6(String),
}

// 3rd
#[derive(Debug)]
enum IpAddr3 {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main(){
    println!("# Ip Address");
    println!("## First version");
    let home = IpAddr1 {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr1 {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    println!("home: {:?}", home);
    println!("loopback: {:?}", loopback);

    println!("## Second version");
    let home = IpAddr2::V4(String::from("127.0.0.1"));
    let loopback = IpAddr2::V6(String::from("::1"));
    println!("home: {:?}", home);
    println!("loopback: {:?}", loopback);

    println!("## Third version");
    let home = IpAddr3::V4(127,0,0,1);
    let loopback = IpAddr3::V6(String::from("::1"));
    println!("home: {:?}", home);
    println!("loopback: {:?}", loopback);
}
