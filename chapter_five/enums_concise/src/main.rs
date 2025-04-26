#[derive(Debug)]
enum VersaoIp {
    V4(u8,u8,u8,u8), //isso anula a necessidade de ter uma struct adicional
    V6(String), //Fazendo assim, quer dizer que cada versão tem um tipo associado
                //usando structs não seria tão conveniente
}

fn main() {
    let loopback_ipv4 = VersaoIp::V4(127,0,0,1);
    let loopback_ipv6 = VersaoIp::V6(String::from("::1"));
    println!(
        "{:#?} {:#?}",
        loopback_ipv4,
        loopback_ipv6,
    );
}
