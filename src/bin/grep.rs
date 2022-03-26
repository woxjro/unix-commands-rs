use clap::Parser;
use std::fs::File;
use std::io::BufReader;

#[derive(Debug)]
struct CustomError(String);

#[derive(Parser, Debug)]
struct Cli {
    pattern: String,
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    let f = File::open(&args.path).expect("could not read file");
    let content = BufReader::new(f);

    unix_commands::grep(content, &args.pattern, &mut std::io::stdout());

    Ok(())
}
