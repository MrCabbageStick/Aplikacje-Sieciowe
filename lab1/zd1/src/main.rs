const OUTPUT_FILE_NAME: &str = "lab1zad1.txt";

fn main() -> Result<(), ()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("No filepath given");
        return Err(());
    }

    let file_path = &args[1];

    match std::fs::copy(file_path, OUTPUT_FILE_NAME) {
        Ok(_) => {
            println!("File '{file_path}' copied to '{OUTPUT_FILE_NAME}'");
            Ok(())
        }
        Err(e) => {
            println!("Unable to copy file '{file_path}'\nError: {e}");
            Err(())
        }
    }
}
