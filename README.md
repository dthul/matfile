# Matfile

Matfile is a library for reading (and in the future writing) Matlab ".mat" files.

__Please note__: This library is still alpha quality software and only implements a subset of the features supported by .mat files.

## Feature Status

Matfile currently allows you to load numeric arrays from .mat files (all floating point and integer types, including complex numbers). All other types are currently ignored.

* [ ] Loading .mat files
  * [x] Numeric arrays
  * [ ] Cell arrays
  * [ ] Structure arrays
  * [ ] Object arrays
  * [ ] Character arrays
  * [ ] Sparse arrays
* [ ] Writing .mat files

## Examples

Loading a .mat file from disk and accessing one of its arrays by name:

```rust
let file = std::fs::File::open("data.mat")?;
let mat_file = matfile::MatFile::parse(file)?;
let pos = mat_file.find_by_name("pos");
println!("{:#?}", pos);
```
Might output something like:
```rust
Some(
    Matrix {
        name: "pos",
        size: [
            2,
            3
        ],
        data: Double {
            real: [
                -5.0,
                8.0,
                6.0,
                9.0,
                7.0,
                10.0
            ],
            imag: None
        }
    }
)
```
Note that data is stored in column-major format. For higher dimensions that means that the first dimension has the fastest varying index.