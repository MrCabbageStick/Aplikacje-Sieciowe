fn main() -> Result<(), ()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("No ip address given");
        return Err(());
    }

    let address = &args[1];

    let octets: Vec<&str> = address.split('.').collect();

    let mut correct = true;

    correct &= octets.len() == 4;
    // Check if all octets fit u8
    correct &= octets
        .iter()
        .map(|octet| octet.parse::<u8>())
        .all(|octet| matches!(octet, Ok(_)));

    if correct {
        println!("Adrress '{address}' is a correct IPv4 address");
        Ok(())
    } else {
        println!("Adrress '{address}' is NOT a correct IPv4 address");
        Err(())
    }
}
