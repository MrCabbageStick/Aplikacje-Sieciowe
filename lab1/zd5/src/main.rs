fn main() -> Result<(), ()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("No hostname given");
        return Err(());
    }

    let hostname = &args[1];

    match dns_lookup::lookup_host(hostname) {
        Ok(addresses) => {
            let addresses = addresses
                .map(|addr| addr.to_string())
                .collect::<Vec<_>>()
                .join("\n");

            println!("Adresses for hostname '{hostname}':\n{addresses}");
            Ok(())
        }
        Err(_) => {
            println!("Unable to find addresses for hostname '{hostname}'");
            Err(())
        }
    }
}
