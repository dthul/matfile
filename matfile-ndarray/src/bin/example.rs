use matfile::MatFile;
use matfile_ndarray::*;
use ndarray as nd;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = include_bytes!("../../../tests/multidimensional.mat");
    let mat_file = crate::MatFile::parse(data.as_ref()).unwrap();
    if let Some(array_a) = mat_file.find_by_name("A") {
        let arr: nd::ArrayView3<'_, f64> = array_a.try_into()?;
        println!("{:#?}", arr);
    }
    Ok(())
}
