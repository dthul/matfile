use matfile::MatFile;

fn main() {
    let data = include_bytes!("../../tests/two_matrices.mat");
    let mat_file = crate::MatFile::parse(data.as_ref()).unwrap();
    println!("{:#?}", mat_file);
}
