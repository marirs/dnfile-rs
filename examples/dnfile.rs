
pub fn main() -> dnfile::Result<()>{
    for (i, arg) in std::env::args().enumerate() {
        if i == 1 {
            let path = std::path::Path::new(arg.as_str());
            let buffer = std::fs::read(path)?;
            let df = dnfile::DnPe::new(arg.as_str(), &buffer)?;
            println!("{:#02x?}", df);
        }
    }
    Ok(())
}
