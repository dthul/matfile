use matfile::MatFile;
#[cfg(feature = "ndarray")]
use ndarr as ndarray;
#[cfg(feature = "ndarray")]
use num_complex;
#[cfg(feature = "ndarray")]
use std::convert::TryInto;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = include_bytes!("../../tests/single_complex.mat");
    let mat_file = crate::MatFile::parse(data.as_ref())?;
    println!("{:#?}", mat_file);
    #[cfg(feature = "ndarray")]
    {
        let mf_arr = mat_file.find_by_name("C").expect("Missing matrix");
        let nd_arr: ndarray::Array2<num_complex::Complex<f32>> = mf_arr.try_into()?;
        println!("{:#?}", nd_arr);
    }
    Ok(())
}
