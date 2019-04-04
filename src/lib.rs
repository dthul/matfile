#![doc(html_root_url = "https://docs.rs/matfile/0.0.1")]

#[macro_use]
extern crate enum_primitive_derive;

mod parse;

#[derive(Clone, Debug)]
pub struct MatFile {
    matrices: Vec<Matrix>,
}

#[derive(Clone, Debug)]
pub struct Matrix {
    dims: parse::Dimensions,
    name: String,
    data: NumericData,
}

#[derive(Clone, Debug)]
pub enum NumericData {
    Int8(Vec<i8>, Option<Vec<i8>>),
    UInt8(Vec<u8>, Option<Vec<u8>>),
    Int16(Vec<i16>, Option<Vec<i16>>),
    UInt16(Vec<u16>, Option<Vec<u16>>),
    Int32(Vec<i32>, Option<Vec<i32>>),
    UInt32(Vec<u32>, Option<Vec<u32>>),
    Int64(Vec<i64>, Option<Vec<i64>>),
    UInt64(Vec<u64>, Option<Vec<u64>>),
    Single(Vec<f32>, Option<Vec<f32>>),
    Double(Vec<f64>, Option<Vec<f64>>),
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
            (parse::NumericData::Double(real), Some(parse::NumericData::Double(imag))) => {
                Ok(NumericData::Double(real, Some(imag)))
            }
            (parse::NumericData::Double(real), None) => Ok(NumericData::Double(real, None)),
            (parse::NumericData::Single(real), Some(parse::NumericData::Single(imag))) => {
                Ok(NumericData::Single(real, Some(imag)))
            }
            (parse::NumericData::Single(real), None) => Ok(NumericData::Single(real, None)),
            (parse::NumericData::UInt64(real), Some(parse::NumericData::UInt64(imag))) => {
                Ok(NumericData::UInt64(real, Some(imag)))
            }
            (parse::NumericData::UInt64(real), None) => Ok(NumericData::UInt64(real, None)),
            (parse::NumericData::Int64(real), Some(parse::NumericData::Int64(imag))) => {
                Ok(NumericData::Int64(real, Some(imag)))
            }
            (parse::NumericData::Int64(real), None) => Ok(NumericData::Int64(real, None)),
            (parse::NumericData::UInt32(real), Some(parse::NumericData::UInt32(imag))) => {
                Ok(NumericData::UInt32(real, Some(imag)))
            }
            (parse::NumericData::UInt32(real), None) => Ok(NumericData::UInt32(real, None)),
            (parse::NumericData::Int32(real), Some(parse::NumericData::Int32(imag))) => {
                Ok(NumericData::Int32(real, Some(imag)))
            }
            (parse::NumericData::Int32(real), None) => Ok(NumericData::Int32(real, None)),
            (parse::NumericData::UInt16(real), Some(parse::NumericData::UInt16(imag))) => {
                Ok(NumericData::UInt16(real, Some(imag)))
            }
            (parse::NumericData::UInt16(real), None) => Ok(NumericData::UInt16(real, None)),
            (parse::NumericData::Int16(real), Some(parse::NumericData::Int16(imag))) => {
                Ok(NumericData::Int16(real, Some(imag)))
            }
            (parse::NumericData::Int16(real), None) => Ok(NumericData::Int16(real, None)),
            (parse::NumericData::UInt8(real), Some(parse::NumericData::UInt8(imag))) => {
                Ok(NumericData::UInt8(real, Some(imag)))
            }
            (parse::NumericData::UInt8(real), None) => Ok(NumericData::UInt8(real, None)),
            (parse::NumericData::Int8(real), Some(parse::NumericData::Int8(imag))) => {
                Ok(NumericData::Int8(real, Some(imag)))
            }
            (parse::NumericData::Int8(real), None) => Ok(NumericData::Int8(real, None)),
            _ => return Err(Error::InternalError),
        }
    }
}

#[derive(Debug)]
pub enum Error {
    IOError(std::io::Error),
    ParseError,
    ConversionError,
    InternalError,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::IOError(_) => write!(f, "An I/O error occurred"),
            Error::ParseError => write!(f, "An error occurred while parsing the file"),
            Error::ConversionError => write!(f, "An error occurred while converting number formats"),
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

impl Matrix {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn shape(&self) -> &parse::Dimensions {
        &self.dims
    }

    pub fn data(&self) -> &NumericData {
        &self.data
    }
}

impl MatFile {
    pub fn parse<R: std::io::Read>(mut reader: R) -> Result<Self, Error> {
        let mut buf = Vec::new();
        reader
            .read_to_end(&mut buf)
            .map_err(|err| Error::IOError(err))?;
        let (_remaining, parse_result) = parse::parse_all(&buf).map_err(|_err| Error::ParseError)?;
        let matrices: Result<Vec<Matrix>, Error> = parse_result
            .data_elements
            .into_iter()
            .filter_map(|data_element| match data_element {
                parse::DataElement::NumericMatrix(flags, dims, name, real, imag) => {
                    let numeric_data = match NumericData::try_from(flags.class, real, imag) {
                        Ok(numeric_data) => numeric_data,
                        Err(err) => return Some(Err(err)),
                    };
                    Some(Ok(Matrix {
                        dims: dims,
                        name: name,
                        data: numeric_data,
                    }))
                }
                _ => None,
            })
            .collect();
        let matrices = matrices?;
        Ok(MatFile { matrices: matrices })
    }

    pub fn matrices(&self) -> &Vec<Matrix> {
        &self.matrices
    }

    pub fn find_by_name<'me>(&'me self, name: &'_ str) -> Option<&'me Matrix> {
        for matrix in &self.matrices {
            if matrix.name == name {
                return Some(matrix);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn double_matrix() {
        let data = include_bytes!("../tests/double.mat");
        let mat_file = MatFile::parse(data.as_ref()).unwrap();
        println!("{:#?}", mat_file);
    }

}
