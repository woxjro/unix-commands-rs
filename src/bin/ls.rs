use clap::Parser;

#[derive(Debug)]
struct CustomError(String);

#[derive(Parser, Debug)]
struct Cli {
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    unix_commands::ls(&args.path, &mut std::io::stdout()).unwrap();

    Ok(())
}
