use martor_core::sha256_hex;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let args: Vec<&str> = args.iter().map(String::as_str).collect();
    match args.as_slice() {
        ["--version"] => println!("martor {}", env!("CARGO_PKG_VERSION")),
        ["sha256", path] => match std::fs::read(path) {
            Ok(bytes) => println!("{}  {}", sha256_hex(&bytes), path),
            Err(e) => {
                eprintln!("{path}: {e}");
                std::process::exit(1);
            }
        },
        _ => {
            eprintln!("usage: martor sha256 FILE | --version");
            std::process::exit(2);
        }
    }
}
