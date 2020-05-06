#![deny(missing_docs)]
//! # SQLite Varint
//! Utility functions for dealing with SQLite varints.

/// Reads a slice of bytes from the start assuming it is a varint.
/// Gives back the varint as i64 and the amount of bytes the varint was big.
/// # Example
/// ```
/// use sqlite_varint::read_varint;
/// // Small positive integer values take little place
/// assert_eq!((1, 1), read_varint(&vec![0b00000001]));
///
/// // Negative values will always take 9 bytes.
/// // Note that the vector is 10 bytes long.
/// assert_eq!((-1, 9), read_varint(&vec![0xff; 10]));
/// ```
pub fn read_varint(bytes: &[u8]) -> (i64, usize) {
    let mut varint: i64 = 0;
    let mut bytes_read: usize = 0;
    for (i, byte) in bytes.iter().enumerate().take(9) {
        bytes_read += 1;
        if i == 8 {
            varint = (varint << 8) | *byte as i64;
            break;
        } else {
            varint = (varint << 7) | (*byte & 0b0111_1111) as i64;
            if *byte < 0b1000_0000 {
                break;
            }
        }
    }
    (varint, bytes_read)
}

/// Read how long the varint would be in amount of bytes.
/// # Example
/// ```
/// use sqlite_varint::read_varint_byte_length;
/// // Small positive integer values take little place
/// assert_eq!(1, read_varint_byte_length(&vec![0b00000001]));
///
/// // Negative values will always take 9 bytes.
/// // Note that the vector is 10 bytes long.
/// assert_eq!(9, read_varint_byte_length(&vec![0xff; 10]));
///
/// ```
pub fn read_varint_byte_length(bytes: &[u8]) -> usize {
    for (i, byte) in bytes.iter().enumerate().take(9) {
        if *byte < 0b1000_0000 {
            return i + 1;
        }
    }
    9
}

/// Serializes an i64 to a variable length byte representation.
/// # Example
/// ```
/// use sqlite_varint::serialize_to_varint;
/// assert_eq!(vec![0b00000001], serialize_to_varint(1));
///
/// assert_eq!(vec![0xff; 9], serialize_to_varint(-1));
///
/// ```
pub fn serialize_to_varint(input: i64) -> Vec<u8> {
    use std::collections::VecDeque;

    let mut result: VecDeque<u8> = VecDeque::new();
    let mut shifted_input = input;

    if input as u64 > 0x00ff_ffff_ffff_ffff {
        // we first push the entire last byte
        result.push_front((shifted_input & 0b1111_1111) as u8);
        shifted_input >>= 8;
    }
    for _ in 0..8 {
        result.push_front((shifted_input & 0b0111_1111) as u8);

        shifted_input >>= 7;
        if result.len() > 1 {
            let p = result.front_mut().unwrap();
            *p |= 0b1000_0000;
        }
        if shifted_input == 0 {
            // we reached the last one in case we don't use all 9 bytes.
            break;
        }
    }

    result.into_iter().collect()
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
    fn serialize_simple_varints() {
        assert_eq!(vec![0b00000001], serialize_to_varint(1));
        assert_eq!(vec![0b00000011], serialize_to_varint(3));
    }

    #[test]
    fn serialize_medium_length_varints() {
        assert_eq!(
            vec![0b10000010, 0b00000001],
            serialize_to_varint(0b100000001)
        )
    }

    #[test]
    fn serialize_negative_varints() {
        assert_eq!(vec![0xff; 9], serialize_to_varint(-1));
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

    #[test]
    fn test_noop() {
        // doing serialze and deserialze should give the input back.:
        let inputs: Vec<i64> = vec![
            0x01,
            0x1ff,
            0x123456,
            0x11223344,
            0x1122334455,
            0x112233445566,
            0x11223344556677,
            0x1928374655647382,
        ];
        for input in inputs {
            assert_eq!(input, read_varint(&serialize_to_varint(input)).0);
        }
    }
}
