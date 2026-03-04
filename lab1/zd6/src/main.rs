fn main() -> Result<(), ()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("No address given");
        return Err(());
    }

    let address = &args[1];

    let connect_to = if args.len() >= 3 {
        format!("{}:{}", address, args[2])
    } else {
        if address.split(':').count() < 2 {
            println!("No port given");
            return Err(());
        }
        address.to_string()
    };

    let Ok(_stream) = std::net::TcpStream::connect(&connect_to) else {
        println!("Unable to connect to '{connect_to}'");
        return Err(());
    };

    println!("Connected to '{connect_to}'");
    Ok(())
}
