128 bytes header
    116 bytes of text
        first 4 bytes non zero indicate Level 5 format, otherwise Level 4
    117-124 subsystem specific data
    125-126 set to 0x0100 when creating the matrix
    127-128 "MI" characters, endian indicator
data element+
    8 bytes tag
        1-4 data type
        5-8 number of bytes of the data in the element (excluding the tag)
    data
    // small element: size < 4 bytes, squeezé dans le tag: 2 bytes data type, 2 bytes size, 4 bytes data;
    // se voit car 2 premiers bytes non nuls (data type occupe que 2 bytes en temps normal
        regular datatype: just determined by the length
        compount datatype: matrix, composed of subelements
            element byte size includes all subelements
            subelements have their own tags

            Numeric/Character Array: 4(+1) subelements
                Array Flags
                    1-2 bytes: undef
                    3 flags (1-4 undef, 5 complex?, 6 global?, 7 logical?, 8 undef)
                    4 class (cell, struct, object, char, sparse, or numeric type)
                Dimensions Array
                    size of each dimension, in an n-sized array of 32 bit values; **numeric arrays have at least 2 dim**
                Array Name
                    name assigned to the array
                Real part
                    numeric data
                (Imaginary part)

# Code
parse_all
  - parse_header
  - parse_next_data_element
    - parse_data_element_tag
    | parse_matrix_data_element
      - parse_array_flags_subelement
      - parse_dimensions_array_subelement
      - parse_array_name_subelement
      | parse_numeric_matrix_data_element
    | parse_compressed_data_element
    | parse_sparse_marix_data_element
      - parse_array_flags_subelement
      - parse_dimensions_array_subelement
      - parse_array_name_subelement
      * parse_row_index_subelement
      * parse_column_index_subelement
      ? parse_numeric_matrix_data_element
