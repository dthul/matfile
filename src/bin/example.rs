use matfile::MatFile;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = include_bytes!("../../tests/double.mat");
    let mat_file = crate::MatFile::parse(data.as_ref())?;
    println!("{:#?}", mat_file);
    Ok(())
}
