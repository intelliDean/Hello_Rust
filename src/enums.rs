
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6
}
#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String
}

pub fn routes(ip_kind: IpAddrKind) {
    println!("IP Address Kind: {:?}", ip_kind);
}


pub fn enums() {

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    routes(four)
}


pub fn using_struct() {

    let home = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("127.0.0.1")
    };

    println!("Home: {:?}", home);

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1")
    };

    println!("Loopback: {:?}", loopback);
}

pub fn using_enum() {
    #[derive(Debug)]
    enum IpAddr {
        V4(String),
        V6(String)
    }

    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    println!("Home (Enum): {:?}", home);
    println!("Loopback (Enum): {:?}", loopback);
}

pub fn enhanced_enums() {
    #[derive(Debug)]
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String)
    }

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    println!("Home (Enhanced): {:?}", home);
    println!("Loopback (Enhanced): {:?}", loopback);

}

