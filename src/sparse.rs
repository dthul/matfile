use nom::{
    apply, be_f32, be_f64, be_i8, be_u8, cond, count, do_parse, i16, i32, i64, le_f32, le_f64,
    le_i8, le_u8, map, switch, take, u16, u32, u64, value, IResult,
};

use crate::parse::*;
use crate::*;

pub fn parse_sparse_matrix_subelements(
    i: &[u8],
    endianness: nom::Endianness,
    flags: ArrayFlags,
) -> IResult<&[u8], DataElement> {
    // Figure out the type of array
    do_parse!(
        i,
        dimensions: apply!(parse_dimensions_array_subelement, endianness)
            >> name: apply!(parse_array_name_subelement, endianness)
            >> row_index: count!(i32!(endianness), flags.nzmax)
            >> column_index: count!(i32!(endianness), 1 + dimensions[1] as usize)
            >> real_part: apply!(parse_numeric_data_element, endianness)
//FIXME test data_type is numeric                   value!(flags.class.is_numeric()),
            >> imag_part: cond!(flags.complex, apply!(parse_numeric_data_element, endianness))
            >> (DataElement::SparseMatrix(
                flags,
                dimensions,
                name,
                row_index.iter().map(|&i| i as usize).collect(),
                column_index.iter().map(|&i| i as usize).collect(),
                real_part,
                imag_part
            ))
    )
}

pub fn parse_numeric_data_element(
    i: &[u8],
    endianness: nom::Endianness,
) -> IResult<&[u8], RawNumericData> {
    do_parse!(
        i,
        tag: apply!(parse_data_element_tag, endianness)
        >> n_entries: value!(tag.data_type.byte_size().unwrap() / tag.data_byte_size as usize)
        >> numeric_data:
        switch!(value!(tag.data_type),
                DataType::Int8 => map!(count!(match endianness {
                        nom::Endianness::Big => be_i8,
                        nom::Endianness::Little => le_i8,
                    }, n_entries)
                    , RawNumericData::Int8)
    | DataType::UInt8 => map!(count!(match endianness {
                        nom::Endianness::Big => be_u8,
                        nom::Endianness::Little => le_u8,
                    }
        , n_entries), RawNumericData::UInt8)
    | DataType::Int16 => map!(count!(i16!(endianness), n_entries), RawNumericData::Int16)
    | DataType::UInt16 => map!(count!(u16!(endianness), n_entries), RawNumericData::UInt16)
    | DataType::Int32 => map!(count!(i32!(endianness), n_entries), RawNumericData::Int32)
    | DataType::UInt32 => map!(count!(u32!(endianness), n_entries), RawNumericData::UInt32)
    | DataType::Single => map!(count!(match endianness {
                        nom::Endianness::Big => be_f32,
                        nom::Endianness::Little => le_f32,
                    }, n_entries), RawNumericData::Single)
    | DataType::Double => map!(count!(match endianness {
                        nom::Endianness::Big => be_f64,
                        nom::Endianness::Little => le_f64,
                    }, n_entries), RawNumericData::Double)
    | DataType::Int64 => map!(count!(i64!(endianness), n_entries), RawNumericData::Int64)
    | DataType::UInt64 => map!(count!(u64!(endianness), n_entries), RawNumericData::UInt64)
    )
    >> take!(tag.padding_byte_size)
    // not accounting for non numeric data
>> (numeric_data)
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sparse() {
        let data = include_bytes!("../tests/bcspwr01.mat");

        let par = parse_header(data);
        dbg!(par);
    }
}
