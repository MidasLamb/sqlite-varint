/// Reads a slice of bytes from the start assuming it is a varint.
/// Gives back the varint as i64 and the amount of bytes the varint was big.
/// #Example
/// ```
/// use sqlite_varint::read_varint;
/// // Small positive integer values take little place
/// assert_eq!((1, 1), read_varint(&vec![0b00000001]));
///
/// // Negative values will always take 9 bytes.
/// assert_eq!((-1, 9), read_varint(&vec![0xff; 10]));
/// ```
pub fn read_varint(bytes: &[u8]) -> (i64, usize) {
    let mut varint: i64 = 0;
    let mut bytes_read: usize = 0;
    for i in 0..9 {
        bytes_read += 1;
        if i == 8 {
            varint = (varint << 8) | bytes[i] as i64;
            break;
        } else {
            varint = (varint << 7) | (bytes[i] & 0b01111111) as i64;
            if bytes[i] < 0b10000000 {
                break;
            }
        }
    }
    (varint, bytes_read)
}

///
pub fn read_varint_byte_length(bytes: &[u8]) -> usize {
    for i in 0..9 {
        if bytes[i] < 0b10000000 {
            return i + 1;
        }
    }
    9
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn read_single_byte_varint() {
        assert_eq!((1, 1), read_varint(&vec![0b00000001]));
        assert_eq!((3, 1), read_varint(&vec![0b00000011]));
        assert_eq!((7, 1), read_varint(&vec![0b00000111]));
        assert_eq!((15, 1), read_varint(&vec![0b00001111]));
    }

    #[test]
    fn read_two_byte_varint() {
        assert_eq!((128, 2), read_varint(&vec![0b10000001, 0b00000000]));
        assert_eq!((129, 2), read_varint(&vec![0b10000001, 0b00000001]));
        assert_eq!((255, 2), read_varint(&vec![0b10000001, 0b01111111]));
    }

    #[test]
    fn read_nine_byte_varint() {
        assert_eq!((-1, 9), read_varint(&vec![0xff; 9]));
    }

    #[test]
    fn read_varint_in_longer_bytes() {
        assert_eq!((1, 1), read_varint(&vec![0x01; 10]));
        assert_eq!((-1, 9), read_varint(&vec![0xff; 10]));
    }

    #[test]
    fn read_varint_lengths() {
        let bytes_vec: Vec<Vec<u8>> = vec![
            vec![0x0f],
            vec![0xff, 0x0f],
            vec![0xff, 0xff, 0x0f],
            vec![0xff, 0xff, 0xff, 0x0f],
            vec![0xff, 0xff, 0xff, 0xff, 0x0f],
            vec![0xff, 0xff, 0xff, 0xff, 0xff, 0x0f],
            vec![0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x0f],
            vec![0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x0f],
            vec![0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x0f],
            // Next ones are exceeding the max length of a varint
            vec![0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x0f, 0x0f],
            vec![
                0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x0f, 0x0f, 0x0f,
            ],
        ];

        let expected_lengths = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 9, 9];

        for i in 0..bytes_vec.len() {
            assert_eq!(expected_lengths[i], read_varint_byte_length(&bytes_vec[i]));
        }
    }
}
