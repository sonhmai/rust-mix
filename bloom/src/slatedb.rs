use std::hash::{DefaultHasher, Hash, Hasher};

use bytes::Bytes;

/// BloomFilter gives a FIRM no or a PROBABLY yes using probabilistic data structure.
struct SlatedbBloomFilter {
    bit_set: Vec<bool>,
    /// Number of hash functions used to determine the bit to check for key existence.
    /// https://en.wikipedia.org/wiki/Bloom_filter#Optimal_number_of_hash_functions.
    num_probes: u32,
}

impl SlatedbBloomFilter {
    pub fn new(size: usize, num_probes: u32) -> Self {
        Self {
            bit_set: vec![false; size],
            num_probes,
        }
    }

    pub fn decode(buf: &[u8]) -> SlatedbBloomFilter {
        todo!()
    }

    pub fn encode(&self) -> Bytes {
        todo!()
    }

    pub fn add_key(&mut self, key: &[u8]) {
        let indices = self.hash_key(key);
        for index in indices {
            self.bit_set[index] = true;
        }
    }

    pub fn has_key(&self, key: &[u8]) -> bool {
        let indices = self.hash_key(key);
        for index in indices {
            if !self.bit_set[index] {
                return false;
            }
        }
        true // if all bits are 1, key may be in set (false positive possible)
    }

    pub fn hash_key(&self, key: &[u8]) -> Vec<usize> {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let hash = hasher.finish();
        let size = self.bit_set.len() as u64;
        let mut result = Vec::with_capacity(self.num_probes as usize);
        // double hashing scheme to generate multiple unique indices from a single hash value.
        let h1 = (hash << 32) >> 32;
        let h2 = hash >> 32;
        for i in 0..self.num_probes {
            let index = ((h1.wrapping_add(i as u64 * h2)) % size) as usize;
            result.push(index);
        }
        result
    }
}

/// Set the bit and index bit in buf
fn set_bit(bit: usize, buf: &mut [u8]) {
    let byte = bit / 8;
    let bit_in_byte = bit % 8;
    buf[byte] |= 1 << bit_in_byte;
}

/// Check if bit at index bit is one
fn check_bit(bit: usize, buf: &[u8]) -> bool {
    let byte = bit / 8;
    let bit_in_byte = bit % 8;
    (buf[byte] & (1 << bit_in_byte)) != 0
}

/// Calculate the optimal number of hash functions
fn optimal_num_probes(bits_per_key: u32) -> u16 {
    // bits_per_key * ln(2)
    // https://en.wikipedia.org/wiki/Bloom_filter#Optimal_number_of_hash_functions
    (bits_per_key as f32 * 0.69) as u16
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_bloom_filter_works() {
        let filter_size = 1000;
        let num_probes = 3;
        let mut filter = SlatedbBloomFilter::new(filter_size, num_probes);
        filter.add_key(("apple").as_bytes());
        filter.add_key(("banana").as_bytes());
        filter.add_key(("orange").as_bytes());

        assert!(!filter.has_key("grape".as_bytes()));
        assert!(!filter.has_key("mango".as_bytes()));
        assert!(!filter.has_key("kiwi".as_bytes()));
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
        }
    }

    #[test]
    fn test_check_bits() {
        let num_bytes = 4;
        for i in 0..num_bytes {
            for b in 0..8 {
                let bit = i * 8 + b;
                let mut buf = vec![0u8; num_bytes];
                buf[i] = 1 << b;
                for checked in 0..num_bytes * 8 {
                    let bit_on = check_bit(checked, buf.as_slice());
                    assert_eq!(bit_on, bit == checked);
                }
            }
        }
    }
}
