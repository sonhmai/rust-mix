use bytes::Bytes;

fn main() {
    println!("bloom filter")
}

/// BloomFilter gives a FIRM no or a PROBABLY yes using probabilistic data structure.
struct BloomFilter {
    buffer: Bytes,
}

impl BloomFilter {
    pub fn decode(buf: &[u8]) -> BloomFilter {
        todo!()
    }

    pub fn encode(&self) -> Bytes {
        todo!()
    }


    pub fn has_key(&self, key: &[u8]) -> bool {
        todo!()
    }
}

/// Set the bit and index bit in buf
fn set_bit(bit: usize, buf: &mut [u8]) {
    let byte = bit / 8;
    let bit_in_byte = bit % 8;
    buf[byte] |= 1 << bit_in_byte;
}

#[cfg(test)]
mod tests {
    use bytes::Bytes;

    use crate::set_bit;

    #[test]
    fn filter_not_in_should_be_exact() {

    }

    #[test]
    fn test_set_bits_does_not_unset_bits() {
        let mut buf = vec![0xFFu8; 3];
        for i in 0..24 {
            set_bit(i, &mut buf);
            assert_eq!(buf, vec![0xFFu8; 3]);
        }
    }

    #[test]
    fn test_set_bits_work() {
        let mut buf = vec![0x00u8; 2];
        for i in 0..16 {
            set_bit(i, &mut buf);
            println!("{:?}", buf);
            buf = vec![0x00u8; 2];
        };
    }
}
