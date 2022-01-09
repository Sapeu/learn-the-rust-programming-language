use std::net::{IpAddr, Ipv4Addr};

fn main() {
    let home: IpAddr = "127.0.0.1".parse().unwrap();
    println!("{:}", home);

    assert_eq!(home, IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
}
