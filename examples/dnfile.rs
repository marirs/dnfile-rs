pub fn main() -> dnfile::Result<()> {
    let Some(arg) = std::env::args().nth(1) else {
        eprintln!("usage: dnfile <path-to-.NET-PE>");
        std::process::exit(2);
    };
    let df = dnfile::DnPe::new(arg.as_str())?;
    println!("{df:#02x?}");
    Ok(())
}
