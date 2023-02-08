//! A general-purpose genomics crate for dealing with DNA.

#![warn(missing_docs)]

use std::{convert::TryFrom, fmt::Display, str::FromStr};

// TODO: add a packed module with the PackedDna struct
//
// this struct must have the following:
// 1. A representation that is more memory efficient that simply storing a vector of `Nuc`
// 2. A FromStr implementation (should be case insensitive like the `Nuc` impl)
// 3. A `FromIterator` implementation to construct it from an iterator over `Nuc`s
// 4. A `fn get(&self, idx: usize) -> Nuc` getter for a particular nucleotide
//
// Make sure to unit test and document all elements
// Also, the internal representation of the PackedDna struct should be privately scoped
/// a packed module
pub mod packed {
    use std::collections::HashMap;
    use std::convert::{TryFrom};
    use std::iter::FromIterator;
    use std::str::FromStr;
    use crate::{Nuc, ParseNucError};

    // 1. A representation that is more memory efficient that simply storing a vector of `nuc`
    // Also, the internal representation of the PackedDna struct should be privately scoped
    // https://doc.rust-lang.org/rust-by-example/custom_types/structs.html
    // struct PackedDna(Vec<Nuc>);
    // struct PackedDna {
    /// the PackedDna struct
    pub struct PackedDna {
        // Also, the internal representation of the PackedDna struct should be privately scoped
        // dna: Vec<Nuc>,

        bytes: Vec<u8>,
        length: usize,
    }

    // 2. A FromStr implementation (should be case insensitive like the `nuc` impl)
    impl FromStr for PackedDna {
        // type Err = ParseNucError<String>;
        type Err = ParseNucError<char>;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            // todo!()
            // let mut vec: Vec<Nuc> = Vec::new();

            // for char in s.chars() {
            //     // https://doc.rust-lang.org/stable/std/convert/trait.TryFrom.html
            //     // https://doc.rust-lang.org/rust-by-example/error/result/result_map.html
            //     // https://stackoverflow.com/questions/26368288/how-do-i-stop-iteration-and-return-an-error-when-iteratormap-returns-a-result
            //     // https://doc.rust-lang.org/rust-by-example/error/iter_result.html
            //     match Nuc::try_from(char) {
            //         Ok(nuc) => {
            //             vec.push(nuc)
            //         }
            //         Err(err) => {
            //             Err(err)
            //         }
            //     }
            // }

            // https://stackoverflow.com/questions/26368288/how-do-i-stop-iteration-and-return-an-error-when-iteratormap-returns-a-result
            let result: Result<Vec<Nuc>, ParseNucError<char>> = s.chars().into_iter().map(|c| {
                // Nuc::try_from(c).unwrap()
                Nuc::try_from(c)
            }).collect();
            let vec = result?;
            // println!("{:?}", vec);

            // Ok(PackedDna(vec))
            // Ok(PackedDna { dna: vec })

            Ok(PackedDna::from_nuc_vec(vec))
        }
    }

    // 3. A `FromIterator` implementation to construct it from an iterator over `nuc`s
    impl FromIterator<Nuc> for PackedDna {
        fn from_iter<T: IntoIterator<Item=Nuc>>(iter: T) -> Self {
            // todo!()
            let vec: Vec<Nuc> = Vec::from_iter(iter);
            // PackedDna {
            //     dna: vec
            // }
            PackedDna::from_nuc_vec(vec)
        }
    }

    // 4. A `fn get(&self, idx: usize) -> nuc` getter for a particular nucleotide
    impl PackedDna {
        /// getter for a particular nucleotide
        pub fn get(&self, idx: usize) -> Nuc {
            // todo!()
            // self[0].get(idx)
            // self.dna.get(idx).unwrap().clone()

            let byte_index = idx / 4;
            let i = idx % 4;
            let byte: u8 = self.bytes[byte_index];
            // shift right (2 * i) bit
            let b = (byte >> (2 * i)) & 0b0000_0011;
            match b {
                0b00 => {
                    Nuc::A
                }
                0b01 => {
                    Nuc::C
                }
                0b10 => {
                    Nuc::G
                }
                0b11 => {
                    Nuc::T
                }
                // impossible
                _ => {
                    Nuc::A
                }
            }
        }

        /// A nucleotide counter
        // pub fn counter(s: &str) -> Result<HashMap<Nuc, i32>, ParseNucError<char>> {
        pub fn counter(s: &str) -> Result<Vec<(Nuc, i32)>, ParseNucError<char>> {
            // https://doc.rust-lang.org/std/collections/struct.HashMap.html
            // https://stackoverflow.com/questions/28392008/is-there-a-more-concise-or-declarative-way-to-initialize-a-hashmap
            // let mut map: HashMap<Nuc, i32> = HashMap::new();
            // map.insert(Nuc::A, 0);
            // map.insert(Nuc::C, 0);
            // map.insert(Nuc::G, 0);
            // map.insert(Nuc::T, 0);
            let mut map = "ACGT".chars().map(|c| (Nuc::try_from(c).unwrap(), 0_i32)).collect::<HashMap<_, _>>();
            // let mut map = HashMap::from([
            //     (Nuc::A, 0_i32),
            //     (Nuc::C, 0_i32),
            //     (Nuc::G, 0_i32),
            //     (Nuc::T, 0_i32),
            // ]);

            let packed_data: PackedDna = PackedDna::from_str(s)?;
            // for nuc in packed_data.dna.iter() {
            for i in 0..packed_data.length {
                let nuc = &packed_data.get(i);

                if let Some(x) = map.get_mut(nuc) {
                    *x += 1;
                }
            }
            // map
            // Ok(map)

            // always keep "ACGT" order
            let x = "ACGT".chars().map(|c| (Nuc::try_from(c).unwrap(), 0_i32))
                .map(|(nuc, _)| {
                    (nuc, map.get(&nuc).unwrap_or(&0).clone())
                }).collect::<Vec<(Nuc, i32)>>();
            Ok(x)
        }

        /// use bits implementation
        fn from_nuc_vec(vec: Vec<Nuc>) -> Self {
            let nuc_length = vec.len();
            let mut bytes_length = nuc_length / 4;
            if nuc_length % 4 > 0 {
                bytes_length += 1;
            }
            // println!("nuc length: {}, need {} bytes", nuc_length, bytes_length);
            let mut bytes: Vec<u8> = Vec::new();
            for b in 0..bytes_length {
                let mut byte: u8 = 0b0000_0000;
                for i in 0..4 {
                    if let Some(nuc) = vec.get(b * 4 + i) {
                        println!("{}, {}, {:?}", b, i, nuc);
                        // 0, 0, A
                        // 0, 1, C
                        // 0, 2, C
                        // 0, 3, T
                        // 0b11010100
                        // 1, 0, G
                        // 0b00000010
                        let b: u8 = match nuc {
                            Nuc::A => {
                                0b00
                            }
                            Nuc::C => {
                                0b01
                            }
                            Nuc::G => {
                                0b10
                            }
                            Nuc::T => {
                                0b11
                            }
                        };
                        // shift left (2 * i) bit
                        byte += b << (2 * i);
                    } else {
                        // nuc_length is not
                        // 0b00 but not Nuc::A
                        // use the PackedDna.length to denote the actual length of nucs
                    }
                }
                println!("0b{:08b}", byte);
                bytes.push(byte);
            }
            // println!("{:?}", bytes);

            // Ok(PackedDna { bytes: vec![], length: 0 })
            PackedDna { bytes, length: nuc_length }
        }
    }
}

/// A nucleotide
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Nuc {
    /// Adenine
    A,
    /// Cytosine
    C,
    /// Guanine
    G,
    /// Thymine
    T,
}

/// An error that can occur when parsing a nucleotide.
#[derive(Debug, thiserror::Error)]
#[error("failed to parse nucleotide from {0}")]
pub struct ParseNucError<T: Display>(T);

impl TryFrom<char> for Nuc {
    type Error = ParseNucError<char>;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value.to_ascii_uppercase() {
            'A' => Ok(Self::A),
            'C' => Ok(Self::C),
            'G' => Ok(Self::G),
            'T' => Ok(Self::T),
            _ => Err(ParseNucError(value)),
        }
    }
}

impl FromStr for Nuc {
    type Err = ParseNucError<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let upper = s.to_ascii_uppercase();
        match upper.as_str() {
            "A" => Ok(Self::A),
            "C" => Ok(Self::C),
            "G" => Ok(Self::G),
            "T" => Ok(Self::T),
            _ => Err(ParseNucError(upper)),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;
    use std::str::FromStr;
    // TODO: fill in tests
    use crate::{Nuc, packed};

    #[test]
    fn dna_from_string() {
        // assert!(false)
        let s = String::from("ACCTG");
        let pack_dna = packed::PackedDna::from_str(&s).unwrap();
        assert_eq!(pack_dna.get(0), Nuc::A);
        assert_eq!(pack_dna.get(1), Nuc::C);
        assert_eq!(pack_dna.get(2), Nuc::C);
        assert_eq!(pack_dna.get(3), Nuc::T);
        assert_eq!(pack_dna.get(4), Nuc::G);
    }

    #[test]
    fn tryfrom_char() {
        // assert!(false);
        let c: char = 'A';
        let nuc = Nuc::try_from(c).unwrap();
        assert_eq!(nuc, Nuc::A);
    }

    #[test]
    fn fromstr() {
        // assert!(false);
        let s: String = String::from("T");
        let nuc = Nuc::from_str(&s).unwrap();
        assert_eq!(nuc, Nuc::T);
    }
}
