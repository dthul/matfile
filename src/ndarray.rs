//! Helpers for converting between `matfile::Array` and `ndarray::Array`.
//!
//! While `matfile` arrays abstract over the underlying data type, `ndarray`
//! arrays are parameterized by a concrete data type. Thus the conversions
//! provided are fallible in case the data types are not compatible.
//!
//! # Examples
//!
//! First, bring the `TryInto` trait into scope:
//!
//! ```rust
//! use std::convert::TryInto;
//! ```
//!
//! ## Dynamically dimensioned arrays
//!
//! Converting a `matfile` array `mf_arr` to a dynamic dimension `ndarray` array
//! `nd_arr`:
//! ```rust
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! #     let data = include_bytes!("../tests/multidimensional.mat");
//! #     let mat_file = matfile::MatFile::parse(data.as_ref()).unwrap();
//! #     let mf_arr = &mat_file.arrays()[0];
//! #     use ndarr as ndarray;
//! #     use std::convert::TryInto;
//! let nd_arr: ndarray::ArrayD<f64> = mf_arr.try_into()?;
//! #     Ok(())
//! # }
//! ```
//!
//! ## Statically dimensioned arrays
//!
//! Converting a `matfile` array `mf_arr` to a static dimension `ndarray` array
//! `nd_arr`:
//! ```rust
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! #     let data = include_bytes!("../tests/single_complex.mat");
//! #     let mat_file = matfile::MatFile::parse(data.as_ref()).unwrap();
//! #     let mf_arr = &mat_file.arrays()[0];
//! #     use ndarr as ndarray;
//! #     use num_complex;
//! #     use std::convert::TryInto;
//! let nd_arr: ndarray::Array2<num_complex::Complex<f32>> = mf_arr.try_into()?;
//! #     Ok(())
//! # }
//! ```

use ndarr as nd;
use ndarr::IntoDimension;
use ndarr::ShapeBuilder;
use num_complex::Complex;
use std::convert::TryInto;

#[derive(Debug)]
pub enum Error {
    /// Generated when the shape (number of dimensions and their respective
    /// sizes) do not match
    ShapeError,
    /// Generated when the number formats are incompatible
    TypeError,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::ShapeError => write!(f, "Array shapes do not match"),
            Error::TypeError => write!(f, "Array types are not compatible"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

macro_rules! dynamic_conversions {
    ( $num:ty, $variant:ident ) => {
        impl<'me> TryInto<nd::ArrayViewD<'me, $num>> for &'me crate::Array {
            type Error = Error;
            fn try_into(self) -> Result<nd::ArrayViewD<'me, $num>, Self::Error> {
                match self.data() {
                    crate::NumericData::$variant {
                        ref real,
                        imag: None,
                    } => {
                        let dimension: nd::IxDyn = self.size().clone().into_dimension();
                        nd::ArrayView::from_shape(dimension.set_f(true), real)
                            .map_err(|_err| Error::ShapeError)
                    }
                    _ => Err(Error::TypeError),
                }
            }
        }

        impl TryInto<nd::ArrayD<$num>> for &crate::Array {
            type Error = Error;
            fn try_into(self) -> Result<nd::ArrayD<$num>, Self::Error> {
                match self.data() {
                    crate::NumericData::$variant {
                        ref real,
                        imag: None,
                    } => {
                        let dimension: nd::IxDyn = self.size().clone().into_dimension();
                        nd::Array::from_shape_vec(dimension.set_f(true), real.clone())
                            .map_err(|_err| Error::ShapeError)
                    }
                    _ => Err(Error::TypeError),
                }
            }
        }

        impl TryInto<nd::ArrayD<Complex<$num>>> for &crate::Array {
            type Error = Error;
            fn try_into(self) -> Result<nd::ArrayD<Complex<$num>>, Self::Error> {
                match self.data() {
                    crate::NumericData::$variant {
                        ref real,
                        imag: Some(ref imag),
                    } => {
                        let dimension: nd::IxDyn = self.size().clone().into_dimension();
                        let values = real
                            .iter()
                            .zip(imag.iter())
                            .map(|(&re, &im)| Complex::new(re, im))
                            .collect();
                        nd::Array::from_shape_vec(dimension.set_f(true), values)
                            .map_err(|_err| Error::ShapeError)
                    }
                    _ => Err(Error::TypeError),
                }
            }
        }
    };
}

macro_rules! static_conversions_n {
    ( $num:ty, $variant:ident, $ndims:literal ) => {
        impl<'me> TryInto<nd::ArrayView<'me, $num, nd::Dim<[nd::Ix; $ndims]>>>
            for &'me crate::Array
        {
            type Error = Error;
            fn try_into(
                self,
            ) -> Result<nd::ArrayView<'me, $num, nd::Dim<[nd::Ix; $ndims]>>, Self::Error> {
                let size = self.size();
                if size.len() != $ndims {
                    return Err(Error::ShapeError);
                }
                let mut shape = [0; $ndims];
                shape.copy_from_slice(size);
                match self.data() {
                    crate::NumericData::$variant {
                        ref real,
                        imag: None,
                    } => {
                        let dimension: nd::Dim<[nd::Ix; $ndims]> = shape.into_dimension();
                        nd::ArrayView::from_shape(dimension.set_f(true), real)
                            .map_err(|_err| Error::ShapeError)
                    }
                    _ => Err(Error::TypeError),
                }
            }
        }

        impl TryInto<nd::Array<$num, nd::Dim<[nd::Ix; $ndims]>>> for &crate::Array {
            type Error = Error;
            fn try_into(self) -> Result<nd::Array<$num, nd::Dim<[nd::Ix; $ndims]>>, Self::Error> {
                let size = self.size();
                if size.len() != $ndims {
                    return Err(Error::ShapeError);
                }
                let mut shape = [0; $ndims];
                shape.copy_from_slice(size);
                match self.data() {
                    crate::NumericData::$variant {
                        ref real,
                        imag: None,
                    } => {
                        let dimension: nd::Dim<[nd::Ix; $ndims]> = shape.into_dimension();
                        nd::Array::from_shape_vec(dimension.set_f(true), real.clone())
                            .map_err(|_err| Error::ShapeError)
                    }
                    _ => Err(Error::TypeError),
                }
            }
        }

        impl TryInto<nd::Array<Complex<$num>, nd::Dim<[nd::Ix; $ndims]>>> for &crate::Array {
            type Error = Error;
            fn try_into(
                self,
            ) -> Result<nd::Array<Complex<$num>, nd::Dim<[nd::Ix; $ndims]>>, Self::Error> {
                let size = self.size();
                if size.len() != $ndims {
                    return Err(Error::ShapeError);
                }
                let mut shape = [0; $ndims];
                shape.copy_from_slice(size);
                match self.data() {
                    crate::NumericData::$variant {
                        ref real,
                        imag: Some(ref imag),
                    } => {
                        let dimension: nd::Dim<[nd::Ix; $ndims]> = shape.into_dimension();
                        let values = real
                            .iter()
                            .zip(imag.iter())
                            .map(|(&re, &im)| Complex::new(re, im))
                            .collect();
                        nd::Array::from_shape_vec(dimension.set_f(true), values)
                            .map_err(|_err| Error::ShapeError)
                    }
                    _ => Err(Error::TypeError),
                }
            }
        }
    };
}

macro_rules! static_conversions {
    ( $num:ty, $variant:ident ) => {
        static_conversions_n!($num, $variant, 2);
        static_conversions_n!($num, $variant, 3);
        static_conversions_n!($num, $variant, 4);
        static_conversions_n!($num, $variant, 5);
        static_conversions_n!($num, $variant, 6);
    };
}

macro_rules! all_conversions {
    ( $num:ty, $variant:ident ) => {
        dynamic_conversions!($num, $variant);
        static_conversions!($num, $variant);
    };
}

all_conversions!(f64, Double);
all_conversions!(f32, Single);
all_conversions!(i64, Int64);
all_conversions!(u64, UInt64);
all_conversions!(i32, Int32);
all_conversions!(u32, UInt32);
all_conversions!(i16, Int16);
all_conversions!(u16, UInt16);
all_conversions!(i8, Int8);
all_conversions!(u8, UInt8);
