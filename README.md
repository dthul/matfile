# Matfile

[![LICENSE](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Build Status](https://github.com/dthul/matfile/actions/workflows/rust.yml/badge.svg)](https://github.com/dthul/matfile/actions/workflows/rust.yml)
[![Docs](https://docs.rs/matfile/badge.svg)](https://docs.rs/matfile/)
[![Crates.io Version](https://img.shields.io/crates/v/matfile.svg)](https://crates.io/crates/matfile)
[![Dependency Status](https://deps.rs/repo/github/dthul/matfile/status.svg)](https://deps.rs/repo/github/dthul/matfile)

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
    Array {
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

# `ndarray` support

Helpers for converting between `matfile::Array` and `ndarray::Array` can be enabled with the `ndarray` feature:

```toml
[dependencies]
matfile = { version = "0.5", features = ["ndarray"] }
```

While `matfile` arrays abstract over the underlying data type, `ndarray`
arrays are parameterized by a concrete data type. Thus the conversions
provided are fallible in case the data types are not compatible.

## Examples

First, bring the `TryInto` trait into scope:

```rust
use std::convert::TryInto;
```

## Dynamically dimensioned arrays

Converting a `matfile` array `mf_arr` to a dynamic dimension `ndarray` array
`nd_arr`:
```rust
let nd_arr: ndarray::ArrayD<f64> = mf_arr.try_into()?;
```

## Statically dimensioned arrays

Converting a `matfile` array `mf_arr` to a static dimension `ndarray` array
`nd_arr`:
```rust
let nd_arr: ndarray::Array2<num_complex::Complex<f32>> = mf_arr.try_into()?;
```
