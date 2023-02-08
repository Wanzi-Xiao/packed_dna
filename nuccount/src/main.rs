// TODO: implement a nucleotide counter
//
// command line argument parsing has been provided
// you must use the PackedDna struct you previously implemented
// if there is any functionality you would like to add to PackedDna feel free to do so in the DNA
// crate
//
// If run with `nuccount --dna ACGTTT" it should print the following to stdout:
// ```
// Input: ACGTTT
//
// A: 1
// C: 1
// G: 1
// T: 3
// ```
//
// be sure to exit with informative error messages if the input is invalid

use structopt::StructOpt;
use dna::packed::PackedDna;

/// Count the number of occurrences of each nucleotide in the provided DNA.
#[derive(Debug, StructOpt)]
struct Opts {
    /// The DNA sequence for which we should retrieve a nucleotide count.
    ///
    /// It is case insensitive but only nucleotides A, C, G and T are supported.
    #[structopt(short = "d", long, required = true)]
    dna: String,
}

fn main() {
    let opts = Opts::from_args();
    let dna = opts.dna;
    println!("Input: {}", &dna);
    println!();

    // IntelliJ IDEA
    // Command: -- --dna ACGT
    //             --dna ACSCGTA
    let result = PackedDna::counter(&dna);
    match result {
        Ok(map) => {
            for (nuc, count) in map.iter() {
                println!("{:?}: {}", nuc, count)
            }
        }
        Err(err) => {
            // println!("{err:?}")
            // ParseNucError('S')
            println!("{}", err)
            // failed to parse nucleotide from S
        }
    }
}
