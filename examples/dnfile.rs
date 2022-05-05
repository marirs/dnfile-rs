
pub fn main() -> dnfile::Result<()>{
    for (i, arg) in std::env::args().enumerate() {
        if i == 1 {
            let df = dnfile::DnPe::new(arg.as_str())?;
            println!("{:#02x?}", df);
        }
    }
    Ok(())
}
