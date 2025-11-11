
enum IpAddrKind {
  V4(String),
  V4(u8, u8, u8, u8),
  V6(String),
}

struct IpAddr {
  kind: IpAddrKind,
  address: String,
}

fn route(ip_kind: IpAddrKind) {}

struct Ipv4Addr {
  // --snip--
}

struct Ipv6Addr {
  // --snip--
}

enum IpAddr {
  V4(Ipv4Addr),
  V6(Ipv6Addr),
}

enum Message {
  Quit,
  Move { x: i32, y: i32 },
  Write(String),
  ChangeColor(i32, i32, i32),
}

impl Message {
  fn call(&self) {
    // method body would be defined here
  }
}


fn lessons() {
  let four = IpAddrKind::V4;
  let six = IpAddrKind::V6;

  route(IpAddrKind::V4);
  route(IpAddrKind::V6);

  let home_1 = IpAddr {
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1"),
  };
  let home_2 = IpAddr::V4(String::from("127.0.0.1"));
  let home_3 = IpAddr::V4(127, 0, 0, 1);

  
  let loopback_1 = IpAddr {
    kind: IpAddrKind::V6,
    address: String::from("::1"),
  };
  let loopback_2 = IpAddr::V6("::1");
  let loopback_3 = IpAddr::V6(String::from("::1"));

  let m = Message::Write(String::from("hello"));
  m.call();


}








