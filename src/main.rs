use std::env;

fn main() {
    let args = env::args().collect::<Vec<_>>();

    // Kokeillaan ottaa ensimäinen parametri, ja mapataan se
    // tyyppiin &str slicing-syntaksilla
    let command = args.get(1)
                      .map(|val| &val[..]);

    match command {
        _ => print_help(),
    }
}

fn print_help() {
    println!("usage: ohtuv <command>");
}
