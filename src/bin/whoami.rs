fn main() -> Result<(), Box<dyn std::error::Error>> {
    unix_commands::whoami(&mut std::io::stdout()).unwrap();
    Ok(())
}
