enum IPAddr {
    Ipv4(u8, u8, u8, u8),
    Ipv6(u8, u8, u8, u8, u8, u8, u8, u8),
}

fn main() {
    let localhost: IPAddr = IPAddr::Ipv4(127, 0, 0, 1);
    match localhost {
        IPAddr::Ipv4(a, b, c, d) => {
            println!("{},{},{},{}", a, b, c, d)
        }
        IPAddr::Ipv6(a, b, c, d, e, f, g, h) => {
            println!("{},{},{},{},{},{},{},{}", a, b, c, d, e, f, g, h)
        }
        _ => {
            println!("nothing")
        }
    }
}
