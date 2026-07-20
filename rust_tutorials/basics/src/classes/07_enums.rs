// An Enum is a versatile tool used to represent a type that can take one of several possible variants
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

// Using structs to save our data
#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

#[derive(Debug)]
#[allow(dead_code)]
enum IpAdd {
    V4(String),
    V6(String),
}

#[derive(Debug)]
#[allow(dead_code)]
enum EnhancedIpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[allow(dead_code)]
pub fn enums_class() {
    println!("");
    println!("=== ENUMS ===");
    println!("");

    let _ip_six: IpAddrKind = IpAddrKind::V4;
    let _ip_siz: IpAddrKind = IpAddrKind::V6;

    fn route(ip_kind: IpAddrKind) {
        println!("Ip kind is: {:?}.", ip_kind);
    }

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    let home: IpAddr = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    println!("At {:?}, the IpKind is: {:?}.", home.address, home.kind);

    let office: IpAddr = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    println!("At {:?}, the IpKind is: {:?}.", office.address, office.kind);

    // Using enums: More concise
    println!();
    let home_v4: IpAdd = IpAdd::V4(String::from("127.0.0.1"));
    println!("V4 home address is: {:?}", home_v4);
    let home_v6: IpAdd = IpAdd::V4(String::from("::1"));
    println!("V6 home address is: {:?}", home_v6);

    // Enhanced Enums
    println!();
    let enhanced_home_v4: IpAdd = IpAdd::V4(String::from("127,0,0,1"));
    println!("V4 enhanced home address is: {:?}", enhanced_home_v4);
    let enhanced_home_v6: IpAdd = IpAdd::V4(String::from("::1"));
    println!("V6 enhanced home address is: {:?}", enhanced_home_v6);
}
