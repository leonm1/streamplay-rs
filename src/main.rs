mod discover;
mod play;
mod stream;

fn main() {
    println!("Welcome to streamplay");
    let args: Vec<String> = std::env::args().collect();
    println!("{:#?}", args[1]);

    match args[1].as_str() {
        "play" => match play::run() {
            Ok(_) => {}
            e => {
                eprintln!("Play failed with the following: {:#?}", e);
            }
        }, 
        "discover" => match discover::run() {
            Ok(_) => {},
            e => {
                eprintln!("Discover failed with the following: {:#?}", e);
            }
        },
        "stream" => match stream::run() {
            Ok(_) => {},
            e => {
                eprintln!("Stream failed with the following: {:#?}", e);
            }
        }
        _ => println!("Either 'discover' or 'play' is required as the first argument"),
    }
}