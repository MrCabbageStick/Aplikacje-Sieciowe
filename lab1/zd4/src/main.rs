use std::str::FromStr;

fn main() -> Result<(), ()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("No ip address given");
        return Err(());
    }

    let address = &args[1];

    let Ok(addr) = std::net::IpAddr::from_str(address) else {
        println!("Adrress '{address}' is NOT a correct IP address");
        return Err(());
    };

    match dns_lookup::lookup_addr(&addr) {
        Ok(res) => {
            println!("Hostname for address '{address}' is '{res}'");
            return Ok(());
        }
        Err(_) => {
            println!("Unable to find hostname for address '{address}'");
            return Err(());
        }
    }
}
