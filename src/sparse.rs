use crate::parse::*;
use crate::*;

fn parse_sparse_matrix_data_element(i: &[u8], endianness: nom::Endianness) -> IResult<&[u8], DataElement> {
    // Figure out the type of array
    do_parse!(
        i,
        flags: apply!(parse_array_flags_subelement, endianness)
            >> dimensions: apply!(parse_dimensions_array_subelement, endianness)
            >> name: apply!(parse_array_name_subelement, endianness)
            >> row_index: count!(i32!(endianness), flags.nzmax)
            >> column_index: count!(i32(endianness), dimensions[1])
            >> data_element: 
        switch!(
            value!(flags.class.is_numeric()),
            true => apply!(parse_numeric_matrix_data_element, flags, dimensions, name, endianness)
        )
            >> (data_element)
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
