extern crate time;

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;

#[derive(Debug)]
enum Timespec {
    None,
    Date { day: i32, month: i32, year: i32 },
    Hour { day: i32, month: i32, year: i32, hour: i32 },
    Minute { day: i32, month: i32, year: i32, hour: i32, minute: i32 },
}

#[derive(Debug)]
struct RepositoryQuery {
    file_name: String,
    time: Timespec,
}

#[derive(PartialEq)]
enum PathStatus {
    NotFound,
    Directory,
    File,
}

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
        Some("restore") => restore_file(&args[2..]),
        _ => print_help(),
    }
}

fn init_repository() {
    let mut directory = env::current_dir().unwrap();
    directory.push(".ohtuv");

    match check_path_status(&directory) {
        PathStatus::Directory => {
            println!("The repository has already been initialized.");
            return;
        },
        PathStatus::File => {
            println!("A file called .ohtuv exists in the current folder. Please remove it to use this program.");
            return;
        },
        _ => {},
    }

    println!("Initializing repository in {:?}", &directory);
    match fs::create_dir(&directory) {
        Ok(_) => println!("Finished"),
        Err(_) => println!("Failed. Check your directory permissions."),
    }
}

fn check_path_status<P: AsRef<Path>>(path: P) -> PathStatus {
    match fs::metadata(&path) {
        Ok(metadata) => if metadata.is_file() { PathStatus::File } else { PathStatus::Directory },
        Err(_) => PathStatus::NotFound,
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
    while directory_box.file_name() != None {
        directory_box.push(".ohtuv");
        if check_path_status(&*directory_box) == PathStatus::Directory {
            return Some(directory_box);
        }

        // Popataan ensin .ohtuv pois, sitten uudestaan että päästään ylemmälle tasolle
        directory_box.pop();
        directory_box.pop();
    }
    None
}

fn save_file(input_path : Option<&String>) {
    let save_result = get_validated_input_path(input_path)
                          .and_then(|input| create_output_path(input)
                                    .and_then(|output| fs::copy(input, output).map_err(|_| "Copying the file into the repository failed.")));

    match save_result {
        Ok(_) => println!("File saved in the repository."),
        Err(error_message) => println!("{}", error_message),
    }
}

fn get_validated_input_path(input_path: Option<&String>) -> Result<&String, &str> {
    input_path.ok_or("A file name is required with the save command.")
              .and_then(|file_name| match check_path_status(file_name) {
                    PathStatus::File => Ok(file_name),
                    PathStatus::Directory => Err("You gave a directory as an argument. The path given needs to point to a file."),
                    PathStatus::NotFound => Err("The path given needs to point to a file."),
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
        .and_then(|file_path| match check_path_status(&file_path) {
            PathStatus::NotFound => Ok(file_path),
            _ => Err("The current version of the file already exists in the repository."),
        })
}

fn restore_file(args: &[String]) {
    let result = parse_query_from_args(args).ok_or("Invalid arguments for restore command.")
        .and_then(find_matching_files);

    match result {
        Ok(matches) => if matches.len() == 1 {
            println!("Single match found, restoring.");
            let &(ref input, RepositoryQuery { file_name: ref output, .. }) = &matches[0];

            match fs::copy(input, output) {
                Ok(_) => println!("File restored."),
                Err(_) => println!("Cannot copy file."),
            }
        } else {
            println!("Ambiguous match, please speficy a more accurate time to restore.");
            println!("Found matches:");
            for single_match in matches {
                println!("    {} {}", format_timespec(&single_match.1.time), &single_match.1.file_name);
            }
        },
        Err(message) => println!("Error: {}", message),
    }
}

fn format_timespec(timespec: &Timespec) -> String {
    match *timespec {
        Timespec::Minute{day, month, year, hour, minute} => format!("{}.{}.{} {}.{}", day, month, year, hour, minute),
        _ => "".to_string(),
    }
}

fn parse_query_from_args(args: &[String]) -> Option<RepositoryQuery> {
    args.get(0)
        .and_then(|arg_three| parse_date(arg_three)
                  .and_then(|(d, m, y)| args.get(1)
                            .and_then(|arg_four| parse_hours_minutes(arg_four)
                                      .map(|(h,min)| Timespec::Minute{day:d, month:m, year:y, hour:h, minute:min})
                                      .or_else(|| parse_hours(arg_four).map(|h| Timespec::Hour{day:d, month:m, year:y, hour:h}))
                                      .and_then(|timespec| args.get(2)
                                                .map(|filename| RepositoryQuery {
                                                    file_name: filename.clone(),
                                                    time: timespec
                                                }))
                                      .or_else(|| Some(RepositoryQuery { file_name: (*arg_four).clone(), time: Timespec::Date{day:d, month:m, year:y}}))
                                     )
                           )
                .or_else(|| Some(RepositoryQuery {file_name: (*arg_three).clone(), time: Timespec::None }))
        )
}

fn parse_date(date_arg: &String) -> Option<(i32, i32, i32)> {
    parse_n_integers(&date_arg, 5)
        .map(|dmy| (dmy[0], dmy[1], dmy[2]))
}

fn parse_n_integers(string: &String, n: usize) -> Option<Vec<i32>> {
    let pieces = string.split('.')
                       .take(n)
                       .map(|piece| i32::from_str(&piece))
                       .take_while(|result| result.is_ok())
                       .map(|result| result.unwrap())
                       .collect::<Vec<_>>();

    if pieces.len() != n {
        return None;
    }

    Some(pieces)
}

fn parse_hours_minutes(time_arg: &String) -> Option<(i32, i32)> {
    parse_n_integers(&time_arg, 3)
        .map(|hm| (hm[0], hm[1]))
}

fn parse_hours(time_arg: &String) -> Option<i32> {
    i32::from_str(&time_arg).ok()
}

fn find_matching_files(query: RepositoryQuery) -> Result<Vec<(String, RepositoryQuery)>, &'static str> {
    find_repository_path()
        .ok_or("Repository not found.")
        .and_then(|repository_path| fs::read_dir(*repository_path).map_err(|_| "Cannot read repository directory."))
        .map(|iterator| iterator.filter_map(|current| current.ok()
                                                             .and_then(|entry| entry.path().into_os_string().into_string().ok())
                                                             .and_then(|path| get_query_if_matching(&path, &query)))
                                .collect::<Vec<_>>())
}

fn get_query_if_matching(path: &String, query: &RepositoryQuery) -> Option<(String, RepositoryQuery)> {
    extract_file_name(path).ok()
        .and_then(|file_name| extract_timespec_for_file(&file_name)
                   .ok()
                   .and_then(|timespec| if file_name.ends_with(&query.file_name) && timespecs_match(&timespec, &query.time) {
                       Some((path.clone(), RepositoryQuery { file_name: query.file_name.clone(), time: timespec}))
                   } else {
                       None
                   }))
}

fn extract_timespec_for_file(path: &String) -> Result<Timespec, &'static str> {
    parse_n_integers(&path, 5)
        .map(|pieces| Timespec::Minute{day: pieces[0], month: pieces[1], year: pieces[2], hour: pieces[3], minute: pieces[4]} )
        .ok_or("Invalid format")
}

fn timespecs_match(file_timespec_original: &Timespec, query_timespec: &Timespec) -> bool {
    if let &Timespec::Minute{day: file_day, month: file_month, year: file_year, hour: file_hour, minute: file_minute} = file_timespec_original {
    match *query_timespec {
        Timespec::None => true,
        Timespec::Date{day, month, year} => day == file_day && month == file_month && year == file_year,
        Timespec::Hour{day, month, year, hour} => day == file_day && month == file_month && year == file_year && hour == file_hour,
        Timespec::Minute{day, month, year, hour, minute} => day == file_day && month == file_month && year == file_year && hour == file_hour && minute == file_minute,
    }
    } else {
        false
    }
}

fn print_help() {
    println!("usage: ohtuv <command>");
    println!("Commands:");
    println!("    init - Initializes a repository in the current directory if none exists.");
    println!("    find - Finds and prints the nearest repository up the directory tree.");
    println!("    save <file> - Saves the given file into the repository.");
    println!("    restore [<day>.<month>.<year>[ <hour>[.<minute>]]] <file> - Restores a file from the repository. The time of the saved file can be given at the required level of accuracy, if the match is ambiguous, the program will let you know.");
}
