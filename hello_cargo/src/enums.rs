struct Ipv4Addr {
    ip: String
}

struct Ipv6Addr {
    ip: String
}

enum IpAddr{
    IPV4(Ipv4Addr),
    IPV6(Ipv6Addr)
}

impl IpAddr {
    fn ip(&self) -> String {
        return String::from("不知道");
    }
}

fn main() {
    println!("ip : {}", IpAddr::IPV4(Ipv4Addr {ip: String::from("127.0.0.1")}).ip());
}