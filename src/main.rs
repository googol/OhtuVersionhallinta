use std::env;
use std::fs;

fn main() {
    let args = env::args().collect::<Vec<_>>();

    // Kokeillaan ottaa ensimäinen parametri, ja mapataan se
    // tyyppiin &str slicing-syntaksilla
    let command = args.get(1)
                      .map(|val| &val[..]);

    match command {
        Some("init") => init_repository(),
        _ => print_help(),
    }
}

fn init_repository() {
    let mut directory = env::current_dir().unwrap();
    directory.push(".ohtuv");

    if let Ok(_) = fs::metadata(&directory) {
        println!("The repository has already been initialized.");
        return;
    }

    println!("Initializing repository in {:?}", &directory);
    match fs::create_dir(&directory) {
        Ok(_) => println!("Finished"),
        Err(_) => println!("Failed. Check your directory permissions."),
    }
}

fn print_help() {
    println!("usage: ohtuv <command>");
    println!("Commands:");
    println!("    init - Initializes a repository in the current directory if none exists.");
}
