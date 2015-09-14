extern crate time;

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
        Some("find") => find_repository(),
        Some("save") => save_file(args.get(2)),
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

fn find_repository() {
    match find_repository_path() {
        Some(directory) => println!("Repository is at {:?}.", directory),
        None => println!("Repository not found."),
    }
}

fn find_repository_path() -> Option<Box<PathBuf>> {
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

fn save_file(input_path : Option<&String>) {
    let save_result = get_validated_input_path(input_path)
                          .and_then(|input| create_output_path(input).map(|output| (input, output)))
                          .and_then(|(input, output)| fs::copy(input, output).map_err(|_| "Copying the file into the repository failed."));

    match save_result {
        Ok(_) => println!("File saved in the repository."),
        Err(error_message) => println!("{}", error_message),
    }
}

fn get_validated_input_path(input_path: Option<&String>) -> Result<&String, &str> {
    input_path.ok_or("A file name is required with the save command.")
              .and_then(validate_path_points_to_file)
}

fn validate_path_points_to_file(file_name: &String) -> Result<&String, &str> {
    fs::metadata(&file_name)
        .map_err(|_| "The path given needs to point to a file.")
        .and_then(|metadata| if metadata.is_file() {
                Ok(file_name)
            } else {
                Err("You gave a directory as an argument. The path given needs to point to a file")
            })
}

fn create_output_path(input_path: &String) -> Result<PathBuf, &str> {
    use time::*;

    extract_file_name(input_path)
        .map(|file_name| {
            let now = now();
            let tm = now.strftime("%d.%m.%Y.%H.%M").ok().unwrap();
            format!("{}.{}", tm, file_name)
        })
        .and_then(file_name_in_repository)
}

fn extract_file_name(path: &String) -> Result<String, &'static str> {
    PathBuf::from(path.clone())
        .file_name()
        .and_then(|file_name| file_name.to_str())
        .ok_or("The given path is not a valid file name.")
        .map(|file_name| file_name.to_string())
}

fn file_name_in_repository(file_name: String) -> Result<PathBuf, &'static str> {
    find_repository_path()
        .ok_or("Repository not found.")
        .map(|repository_path| repository_path.join(file_name))
        .and_then(|file_path| match fs::metadata(&file_path) {
          Ok(_) => Err("The current version of the file already exists in the repository."),
          Err(_) => Ok(file_path),
        })
}

fn print_help() {
    println!("usage: ohtuv <command>");
    println!("Commands:");
    println!("    init - Initializes a repository in the current directory if none exists.");
    println!("    find - Finds and prints the nearest repository up the directory tree.");
    println!("    save <file> - Saves the given file into the repository.");
}
