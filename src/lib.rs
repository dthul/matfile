#![doc(html_root_url = "https://docs.rs/matfile/0.2.0")]

//! Matfile is a library for reading (and in the future writing) Matlab ".mat" files.
//! 
//! __Please note__: This library is still alpha quality software and only implements a subset of the features supported by .mat files.
//! 
//! ## Feature Status
//! 
//! Matfile currently allows you to load numeric arrays from .mat files (all floating point and integer types, including complex numbers). All other types are currently ignored.
//! 
//! * [ ] Loading .mat files
//!   * [x] Numeric arrays
//!   * [ ] Cell arrays
//!   * [ ] Structure arrays
//!   * [ ] Object arrays
//!   * [ ] Character arrays
//!   * [ ] Sparse arrays
//! * [ ] Writing .mat files
//! 
//! ## Examples
//! 
//! Loading a .mat file from disk and accessing one of its arrays by name:
//! 
//! ```rust
//! # pub fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let file = std::fs::File::open("tests/double.mat")?;
//! let mat_file = matfile::MatFile::parse(file)?;
//! let pos = mat_file.find_by_name("pos");
//! println!("{:#?}", pos);
//! # Ok(())
//! # }
//! ```
//! Might output something like:
//! ```text
//! Some(
//!     Array {
//!         name: "pos",
//!         size: [
//!             2,
//!             3
//!         ],
//!         data: Double {
//!             real: [
//!                 -5.0,
//!                 8.0,
//!                 6.0,
//!                 9.0,
//!                 7.0,
//!                 10.0
//!             ],
//!             imag: None
//!         }
//!     }
//! )
//! ```
//! Note that data is stored in column-major format. For higher dimensions that means that the first dimension has the fastest varying index.

#[macro_use]
extern crate enum_primitive_derive;

mod parse;

/// MatFile is a collection of named arrays.
/// 
/// You can load a ".mat" file from disk like this:
/// ```rust
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let file = std::fs::File::open("tests/double.mat")?;
/// let mat_file = matfile::MatFile::parse(file)?;
/// # Ok(())
/// # }
/// ```
#[derive(Clone, Debug)]
pub struct MatFile {
    arrays: Vec<Array>,
}

/// A numeric array (the only type supported at the moment).
/// 
/// You can access the arrays of a MatFile either by name or by iterating
/// through all of them:
/// ```rust
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// # let file = std::fs::File::open("tests/double.mat")?;
/// # let mat_file = matfile::MatFile::parse(file)?;
/// if let Some(array_a) = mat_file.find_by_name("A") {
///     println!("Array \"A\": {:#?}", array_a);
/// }
/// 
/// for array in mat_file.arrays() {
///     println!("Found array named {} of size {:?}", array.name(), array.size());
/// }
/// # Ok(())
/// # }
/// ```
#[derive(Clone, Debug)]
pub struct Array {
    name: String,
    size: Vec<usize>,
    data: NumericData,
}

/// Stores the data of a numerical array and abstracts over the actual data
/// type used. Real and imaginary parts are stored in separate vectors with the
/// imaginary part being optional.
/// 
/// Numerical data is stored in column-major order. When talking about higher
/// dimensional arrays this means that the index of the first dimension varies
/// fastest.
#[derive(Clone, Debug)]
pub enum NumericData {
    Int8 {
        real: Vec<i8>,
        imag: Option<Vec<i8>>,
    },
    UInt8 {
        real: Vec<u8>,
        imag: Option<Vec<u8>>,
    },
    Int16 {
        real: Vec<i16>,
        imag: Option<Vec<i16>>,
    },
    UInt16 {
        real: Vec<u16>,
        imag: Option<Vec<u16>>,
    },
    Int32 {
        real: Vec<i32>,
        imag: Option<Vec<i32>>,
    },
    UInt32 {
        real: Vec<u32>,
        imag: Option<Vec<u32>>,
    },
    Int64 {
        real: Vec<i64>,
        imag: Option<Vec<i64>>,
    },
    UInt64 {
        real: Vec<u64>,
        imag: Option<Vec<u64>>,
    },
    Single {
        real: Vec<f32>,
        imag: Option<Vec<f32>>,
    },
    Double {
        real: Vec<f64>,
        imag: Option<Vec<f64>>,
    },
}

fn try_convert_number_format(
    target_type: parse::ArrayType,
    data: parse::NumericData,
) -> Result<parse::NumericData, Error> {
    match target_type {
        parse::ArrayType::Double => match data {
            parse::NumericData::UInt8(data) => Ok(parse::NumericData::Double(
                data.into_iter().map(|x| x as f64).collect(),
            )),
            parse::NumericData::Int16(data) => Ok(parse::NumericData::Double(
                data.into_iter().map(|x| x as f64).collect(),
            )),
            parse::NumericData::UInt16(data) => Ok(parse::NumericData::Double(
                data.into_iter().map(|x| x as f64).collect(),
            )),
            parse::NumericData::Int32(data) => Ok(parse::NumericData::Double(
                data.into_iter().map(|x| x as f64).collect(),
            )),
            parse::NumericData::Double(data) => Ok(parse::NumericData::Double(data)),
            _ => Err(Error::ConversionError),
        },
        parse::ArrayType::Single => match data {
            parse::NumericData::UInt8(data) => Ok(parse::NumericData::Single(
                data.into_iter().map(|x| x as f32).collect(),
            )),
            parse::NumericData::Int16(data) => Ok(parse::NumericData::Single(
                data.into_iter().map(|x| x as f32).collect(),
            )),
            parse::NumericData::UInt16(data) => Ok(parse::NumericData::Single(
                data.into_iter().map(|x| x as f32).collect(),
            )),
            parse::NumericData::Int32(data) => Ok(parse::NumericData::Single(
                data.into_iter().map(|x| x as f32).collect(),
            )),
            parse::NumericData::Single(data) => Ok(parse::NumericData::Single(data)),
            _ => Err(Error::ConversionError),
        },
        parse::ArrayType::UInt64 => match data {
            parse::NumericData::UInt8(data) => Ok(parse::NumericData::UInt64(
                data.into_iter().map(|x| x as u64).collect(),
            )),
            parse::NumericData::Int16(data) => Ok(parse::NumericData::UInt64(
                data.into_iter().map(|x| x as u64).collect(),
            )),
            parse::NumericData::UInt16(data) => Ok(parse::NumericData::UInt64(
                data.into_iter().map(|x| x as u64).collect(),
            )),
            parse::NumericData::Int32(data) => Ok(parse::NumericData::UInt64(
                data.into_iter().map(|x| x as u64).collect(),
            )),
            parse::NumericData::UInt64(data) => Ok(parse::NumericData::UInt64(data)),
            _ => Err(Error::ConversionError),
        },
        parse::ArrayType::Int64 => match data {
            parse::NumericData::UInt8(data) => Ok(parse::NumericData::Int64(
                data.into_iter().map(|x| x as i64).collect(),
            )),
            parse::NumericData::Int16(data) => Ok(parse::NumericData::Int64(
                data.into_iter().map(|x| x as i64).collect(),
            )),
            parse::NumericData::UInt16(data) => Ok(parse::NumericData::Int64(
                data.into_iter().map(|x| x as i64).collect(),
            )),
            parse::NumericData::Int32(data) => Ok(parse::NumericData::Int64(
                data.into_iter().map(|x| x as i64).collect(),
            )),
            parse::NumericData::Int64(data) => Ok(parse::NumericData::Int64(data)),
            _ => Err(Error::ConversionError),
        },
        parse::ArrayType::UInt32 => match data {
            parse::NumericData::UInt8(data) => Ok(parse::NumericData::UInt32(
                data.into_iter().map(|x| x as u32).collect(),
            )),
            parse::NumericData::Int16(data) => Ok(parse::NumericData::UInt32(
                data.into_iter().map(|x| x as u32).collect(),
            )),
            parse::NumericData::UInt16(data) => Ok(parse::NumericData::UInt32(
                data.into_iter().map(|x| x as u32).collect(),
            )),
            parse::NumericData::UInt32(data) => Ok(parse::NumericData::UInt32(data)),
            _ => Err(Error::ConversionError),
        },
        parse::ArrayType::Int32 => match data {
            parse::NumericData::UInt8(data) => Ok(parse::NumericData::Int32(
                data.into_iter().map(|x| x as i32).collect(),
            )),
            parse::NumericData::Int16(data) => Ok(parse::NumericData::Int32(
                data.into_iter().map(|x| x as i32).collect(),
            )),
            parse::NumericData::UInt16(data) => Ok(parse::NumericData::Int32(
                data.into_iter().map(|x| x as i32).collect(),
            )),
            parse::NumericData::Int32(data) => Ok(parse::NumericData::Int32(data)),
            _ => Err(Error::ConversionError),
        },
        parse::ArrayType::UInt16 => match data {
            parse::NumericData::UInt8(data) => Ok(parse::NumericData::UInt16(
                data.into_iter().map(|x| x as u16).collect(),
            )),
            parse::NumericData::UInt16(data) => Ok(parse::NumericData::UInt16(data)),
            _ => Err(Error::ConversionError),
        },
        parse::ArrayType::Int16 => match data {
            parse::NumericData::UInt8(data) => Ok(parse::NumericData::Int16(
                data.into_iter().map(|x| x as i16).collect(),
            )),
            parse::NumericData::Int16(data) => Ok(parse::NumericData::Int16(data)),
            _ => Err(Error::ConversionError),
        },
        parse::ArrayType::UInt8 => match data {
            parse::NumericData::UInt8(data) => Ok(parse::NumericData::UInt8(data)),
            _ => Err(Error::ConversionError),
        },
        parse::ArrayType::Int8 => match data {
            parse::NumericData::Int8(data) => Ok(parse::NumericData::Int8(data)),
            _ => Err(Error::ConversionError),
        },
        _ => Err(Error::ConversionError),
    }
}

impl NumericData {
    fn try_from(
        target_type: parse::ArrayType,
        real: parse::NumericData,
        imag: Option<parse::NumericData>,
    ) -> Result<Self, Error> {
        let real = try_convert_number_format(target_type, real)?;
        let imag = match imag {
            Some(imag) => Some(try_convert_number_format(target_type, imag)?),
            None => None,
        };
        // The next step should never fail unless there is a bug in the code
        match (real, imag) {
            (parse::NumericData::Double(real), None) => Ok(NumericData::Double {
                real: real,
                imag: None,
            }),
            (parse::NumericData::Double(real), Some(parse::NumericData::Double(imag))) => {
                Ok(NumericData::Double {
                    real: real,
                    imag: Some(imag),
                })
            }
            (parse::NumericData::Single(real), None) => Ok(NumericData::Single {
                real: real,
                imag: None,
            }),
            (parse::NumericData::Single(real), Some(parse::NumericData::Single(imag))) => {
                Ok(NumericData::Single {
                    real: real,
                    imag: Some(imag),
                })
            }
            (parse::NumericData::UInt64(real), None) => Ok(NumericData::UInt64 {
                real: real,
                imag: None,
            }),
            (parse::NumericData::UInt64(real), Some(parse::NumericData::UInt64(imag))) => {
                Ok(NumericData::UInt64 {
                    real: real,
                    imag: Some(imag),
                })
            }
            (parse::NumericData::Int64(real), None) => Ok(NumericData::Int64 {
                real: real,
                imag: None,
            }),
            (parse::NumericData::Int64(real), Some(parse::NumericData::Int64(imag))) => {
                Ok(NumericData::Int64 {
                    real: real,
                    imag: Some(imag),
                })
            }
            (parse::NumericData::UInt32(real), None) => Ok(NumericData::UInt32 {
                real: real,
                imag: None,
            }),
            (parse::NumericData::UInt32(real), Some(parse::NumericData::UInt32(imag))) => {
                Ok(NumericData::UInt32 {
                    real: real,
                    imag: Some(imag),
                })
            }
            (parse::NumericData::Int32(real), None) => Ok(NumericData::Int32 {
                real: real,
                imag: None,
            }),
            (parse::NumericData::Int32(real), Some(parse::NumericData::Int32(imag))) => {
                Ok(NumericData::Int32 {
                    real: real,
                    imag: Some(imag),
                })
            }
            (parse::NumericData::UInt16(real), None) => Ok(NumericData::UInt16 {
                real: real,
                imag: None,
            }),
            (parse::NumericData::UInt16(real), Some(parse::NumericData::UInt16(imag))) => {
                Ok(NumericData::UInt16 {
                    real: real,
                    imag: Some(imag),
                })
            }
            (parse::NumericData::Int16(real), None) => Ok(NumericData::Int16 {
                real: real,
                imag: None,
            }),
            (parse::NumericData::Int16(real), Some(parse::NumericData::Int16(imag))) => {
                Ok(NumericData::Int16 {
                    real: real,
                    imag: Some(imag),
                })
            }
            (parse::NumericData::UInt8(real), None) => Ok(NumericData::UInt8 {
                real: real,
                imag: None,
            }),
            (parse::NumericData::UInt8(real), Some(parse::NumericData::UInt8(imag))) => {
                Ok(NumericData::UInt8 {
                    real: real,
                    imag: Some(imag),
                })
            }
            (parse::NumericData::Int8(real), None) => Ok(NumericData::Int8 {
                real: real,
                imag: None,
            }),
            (parse::NumericData::Int8(real), Some(parse::NumericData::Int8(imag))) => {
                Ok(NumericData::Int8 {
                    real: real,
                    imag: Some(imag),
                })
            }
            _ => return Err(Error::InternalError),
        }
    }
}

#[derive(Debug)]
pub enum Error {
    IOError(std::io::Error),
    ParseError(nom::Err<&'static [u8], u32>),
    ConversionError,
    InternalError,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::IOError(_) => write!(f, "An I/O error occurred"),
            Error::ParseError(_) => write!(f, "An error occurred while parsing the file"),
            Error::ConversionError => {
                write!(f, "An error occurred while converting number formats")
            }
            Error::InternalError => write!(f, "An internal error occurred, this is a bug"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::IOError(ref err) => Some(err),
            _ => None,
        }
    }
}

impl Array {
    /// The name of this array.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// The size of this array.
    /// 
    /// The number of entries in this vector is equal to the number of
    /// dimensions of this array. Each array has at least two dimensions.
    /// For two-dimensional arrays the first dimension is the number of rows
    /// while the second dimension is the number of columns.
    pub fn size(&self) -> &Vec<usize> {
        &self.size
    }

    /// The number of dimensions of this array. Is at least two.
    pub fn ndims(&self) -> usize {
        self.size.len()
    }

    /// The actual numerical data stored in this array.
    /// 
    /// ```rust
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let file = std::fs::File::open("tests/double.mat")?;
    /// # let mat_file = matfile::MatFile::parse(file)?;
    /// # let array = &mat_file.arrays()[0];
    /// if let matfile::NumericData::Double { real: real, imag: _ } = array.data() {
    ///     println!("Real part of the data: {:?}", real);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    /// 
    /// For a more convenient access to the data, consider using the
    /// `matfile-ndarray` crate.
    pub fn data(&self) -> &NumericData {
        &self.data
    }
}

impl MatFile {
    /// Tries to parse a byte sequence as a ".mat" file.
    pub fn parse<R: std::io::Read>(mut reader: R) -> Result<Self, Error> {
        let mut buf = Vec::new();
        reader
            .read_to_end(&mut buf)
            .map_err(|err| Error::IOError(err))?;
        let (_remaining, parse_result) = parse::parse_all(&buf)
            .map_err(|err| Error::ParseError(parse::replace_err_slice(err, &[])))?;
        let arrays: Result<Vec<Array>, Error> = parse_result
            .data_elements
            .into_iter()
            .filter_map(|data_element| match data_element {
                parse::DataElement::NumericMatrix(flags, dims, name, real, imag) => {
                    let size = dims.into_iter().map(|d| d as usize).collect();
                    let numeric_data = match NumericData::try_from(flags.class, real, imag) {
                        Ok(numeric_data) => numeric_data,
                        Err(err) => return Some(Err(err)),
                    };
                    Some(Ok(Array {
                        size: size,
                        name: name,
                        data: numeric_data,
                    }))
                }
                _ => None,
            })
            .collect();
        let arrays = arrays?;
        Ok(MatFile { arrays: arrays })
    }

    /// List of all arrays in this .mat file.
    /// 
    /// When parsing a .mat file all arrays of unsupported type (currently all
    /// non-numerical and sparse arrays) will be ignored and will thus not be
    /// part of this list.
    pub fn arrays(&self) -> &Vec<Array> {
        &self.arrays
    }

    /// Returns an array with the given name if it exists. Case sensitive.
    /// 
    /// When parsing a .mat file all arrays of unsupported type (currently all
    /// non-numerical and sparse arrays) will be ignored and will thus not be
    /// returned by this function.
    pub fn find_by_name<'me>(&'me self, name: &'_ str) -> Option<&'me Array> {
        for array in &self.arrays {
            if array.name == name {
                return Some(array);
            }
        }
        None
    }
}

// TODO: improve tests.
// The tests are not very comprehensive yet and they only test whether
// the files can be loaded without error, but not whether the result
// is actually correct.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn double_array() {
        let data = include_bytes!("../tests/double.mat");
        let _mat_file = MatFile::parse(data.as_ref()).unwrap();
    }

    #[test]
    fn double_as_int16_array() {
        let data = include_bytes!("../tests/double_as_int16.mat");
        let _mat_file = MatFile::parse(data.as_ref()).unwrap();
    }

    #[test]
    fn double_as_uint8_array() {
        let data = include_bytes!("../tests/double_as_uint8.mat");
        let _mat_file = MatFile::parse(data.as_ref()).unwrap();
    }

    #[test]
    fn single_complex_array() {
        let data = include_bytes!("../tests/single_complex.mat");
        let _mat_file = MatFile::parse(data.as_ref()).unwrap();
    }

    #[test]
    fn two_arrays() {
        let data = include_bytes!("../tests/two_arrays.mat");
        let _mat_file = MatFile::parse(data.as_ref()).unwrap();
    }

    #[test]
    fn multidimensional_array() {
        let data = include_bytes!("../tests/multidimensional.mat");
        let _mat_file = MatFile::parse(data.as_ref()).unwrap();
    }

    #[test]
    fn long_name() {
        let data = include_bytes!("../tests/long_name.mat");
        let _mat_file = MatFile::parse(data.as_ref()).unwrap();
    }
}
