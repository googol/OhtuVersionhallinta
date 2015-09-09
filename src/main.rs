use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let args = env::args().collect::<Vec<_>>();

    // Kokeillaan ottaa ensimäinen parametri, ja mapataan se
    // tyyppiin &str slicing-syntaksilla
    let command = args.get(1)
                      .map(|val| &val[..]);

    match command {
        Some("init") => init_repository(),
        Some("find") => find_repository_command(),
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

fn find_repository_command() {
    match find_repository() {
        Some(directory) => println!("Repository is at {:?}.", directory),
        None => println!("Repository not found."),
    }
}
fn find_repository() -> Option<Box<PathBuf>> {
    let mut directory_box = Box::new(env::current_dir().unwrap());
    let success = walk_parent_directories(&mut *directory_box);
    if success {
        Some(directory_box)
    } else {
        None
    }
}

fn walk_parent_directories(directory: &mut PathBuf) -> bool {
    while directory.file_name() != None {
        directory.push(".ohtuv");
        if let Ok(_) = fs::metadata(&directory) {
            return true;
        }

        // Popataan ensin .ohtuv pois, sitten uudestaan että päästään ylemmälle tasolle
        directory.pop();
        directory.pop();
    }
    false
}

fn print_help() {
    println!("usage: ohtuv <command>");
    println!("Commands:");
    println!("    init - Initializes a repository in the current directory if none exists.");
    println!("    find - Finds and prints the nearest repository up the directory tree.");
}
