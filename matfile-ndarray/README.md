# matfile-ndarray

Helpers for converting between `matfile::Array` and `ndarray::Array`.

While `matfile` arrays abstract over the underlying data type, `ndarray`
arrays are parameterized by a concrete data type. Thus the conversions
provided are fallible in case the data types are not compatible.

# Examples

First, bring the `TryInto` trait into scope:

```rust
use matfile_ndarray::TryInto;
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
