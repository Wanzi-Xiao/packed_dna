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
mod packed {
    use std::convert::{TryFrom};
    use std::iter::FromIterator;
    use std::str::FromStr;
    use crate::{Nuc, ParseNucError};

    // 1. A representation that is more memory efficient that simply storing a vector of `nuc`
    // Also, the internal representation of the PackedDna struct should be privately scoped
    // struct PackedDna(Vec<Nuc>);
    // struct PackedDna {
    pub struct PackedDna {
        // Also, the internal representation of the PackedDna struct should be privately scoped
        dna: Vec<Nuc>,
    }

    // 2. A FromStr implementation (should be case insensitive like the `nuc` impl)
    impl FromStr for PackedDna {
        // type Err = ParseNucError<String>;
        type Err = ParseNucError<char>;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            // todo!()
            // let mut vec: Vec<Nuc> = Vec::new();

            // for char in s.chars() {
            //     match Nuc::try_from(char) {
            //         Ok(nuc) => {
            //             vec.push(nuc)
            //         }
            //         Err(err) => {
            //             Err(err)
            //         }
            //     }
            // }

            let vec: Vec<Nuc> = s.chars().into_iter().map(|c| {
                Nuc::try_from(c).unwrap()
            }).collect();
            // println!("{:?}", vec);

            // Ok(PackedDna(vec))
            Ok(PackedDna { dna: vec })
        }
    }

    // 3. A `FromIterator` implementation to construct it from an iterator over `nuc`s
    impl FromIterator<Nuc> for PackedDna {
        fn from_iter<T: IntoIterator<Item=Nuc>>(iter: T) -> Self {
            // todo!()
            let vec: Vec<Nuc> = Vec::from_iter(iter);
            PackedDna {
                dna: vec
            }
        }
    }

    // 4. A `fn get(&self, idx: usize) -> nuc` getter for a particular nucleotide
    impl PackedDna {
        pub fn get(&self, idx: usize) -> Nuc {
            // todo!()
            // self[0].get(idx)
            self.dna.get(idx).unwrap().clone()
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
        let s = String::from("ACCT");
        let pack_dna = packed::PackedDna::from_str(&s).unwrap();
        assert_eq!(pack_dna.get(0), Nuc::A);
        assert_eq!(pack_dna.get(1), Nuc::C);
        assert_eq!(pack_dna.get(2), Nuc::C);
        assert_eq!(pack_dna.get(3), Nuc::T);
    }

    #[test]
    fn tryfrom_char() {
        // assert!(false);
        let c: char = 'A';
        let nuc = Nuc::try_from(c).unwrap();
        assert_eq!(nuc, Nuc::A);

        let c: char = 'C';
        let nuc = Nuc::try_from(c).unwrap();
        assert_eq!(nuc, Nuc::C);

        let c: char = 'T';
        let nuc = Nuc::try_from(c).unwrap();
        assert_eq!(nuc, Nuc::T);

        let c: char = 'G';
        let nuc = Nuc::try_from(c).unwrap();
        assert_eq!(nuc, Nuc::G);
    }

    #[test]
    fn fromstr() {
        // assert!(false);
        let s: String = String::from("A");
        let nuc = Nuc::from_str(&s).unwrap();
        assert_eq!(nuc, Nuc::A);

        let s: String = String::from("C");
        let nuc = Nuc::from_str(&s).unwrap();
        assert_eq!(nuc, Nuc::C);

        let s: String = String::from("T");
        let nuc = Nuc::from_str(&s).unwrap();
        assert_eq!(nuc, Nuc::T);

        let s: String = String::from("G");
        let nuc = Nuc::from_str(&s).unwrap();
        assert_eq!(nuc, Nuc::G);
    }
}
