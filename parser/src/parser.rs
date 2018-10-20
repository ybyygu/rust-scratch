// base

// [[file:~/Workspace/Programming/rust-scratch/parser/parser.note::*base][base:1]]
use crate::nom_parser::*;

use std::fs::File;
use std::io::{Read, BufRead, BufReader};
use std::path::{Path};

use quicli::prelude::*;
use nom;

/// A stream parser for large text file
pub struct TextParser {
    /// The buffer size counted in number of lines
    nlines: usize,
}

/// General interface for parsing a large text file
impl Default for TextParser {
    fn default() -> Self {
        TextParser {
            nlines: 100,
        }
    }
}

impl TextParser {
    /// Entry point for parsing a text file
    ///
    /// # Parameters
    /// - parser: nom parser
    /// - collector: a closure to collect parsed results
    pub fn parse<R: Read, F, C, P: Sized>(&self, f: R, parser: F, mut collector: C) -> Result<()>
    where
        F: Fn(&str) -> nom::IResult<&str, P>,
        C: FnMut(P),
    {
        // a. prepare data
        let mut reader = BufReader::new(f);
        let mut chunk = String::new();

        // b. process the read/parse loop
        // indicate if we finish reading
        let mut eof = false;
        'out: loop {
            // 0. fill chunk
            if ! eof {
                debug!("fill data");
                for _ in 0..self.nlines {
                    // reach EOF
                    if reader.read_line(&mut chunk)? == 0 {
                        eof = true;
                        // a workaround for nom 4.0 changes: append a magic_eof line to make
                        // stream `complete`
                        chunk.push_str(MAGIC_EOF);
                        break;
                    }
                }
            }

            // 1. parse/consume the chunk until we get Incomplete error
            // remained: the unprocessed lines by parser
            let mut remained = String::new();
            loop {
                match parser(&chunk) {
                    // 1.1 success parsed one part
                    Ok((rest, part)) => {
                        // save the remained data
                        remained = String::from(rest);
                        // collect the parsed value
                        collector(part);
                    },

                    // 1.2 the chunk is incomplete.
                    // `Incomplete` means the nom parser does not have enough data to decide,
                    // so we wait for the next refill and then retry parsing
                    Err(nom::Err::Incomplete(_)) => {
                        // the chunk is unstained, so just break the parsing loop
                        break;
                    },

                    // 1.3 found parse errors, just ignore it and continue
                    Err(nom::Err::Error(err)) => {
                        eprintln!("found parsing error: {:?}", err);
                        eprintln!("the context lines: {}", chunk);
                        break;
                        // break 'out;
                    },

                    // 1.4 found serious errors
                    Err(nom::Err::Failure(err)) => {
                        bail!("encount hard failure: {:?}", err);
                    },

                    // 1.5 alerting nom changes
                    _ => {
                        bail!("found unrecovered nom state!");
                    }
                }

                // 2. update the chunk with remained data after a successful parsing
                // chunk.clear();
                // chunk.push_str(&remained);
                chunk = remained;
            }

            // all done, get out the loop
            if eof {
                if chunk.len() != 0 {
                    eprintln!("remained data:\n {:}", chunk);
                    warn!("remained data:\n {:}", chunk);
                }
                break
            };

        }
        info!("parsing done.");

        // c. finish the job
        Ok(())
    }
}
// base:1 ends here

// xyz

// [[file:~/Workspace/Programming/rust-scratch/parser/parser.note::*xyz][xyz:1]]
use gchemol::Molecule;

named!(parse_plain_xyz<&str, Molecule>, do_parse!(
    mol: read_molecule_pxyz >>
         blank_line         >>
    (
        mol
    )
));

named!(parse_xyz<&str, Molecule>, do_parse!(
    mol: read_molecule_xyz >>
    (
        mol
    )
));

#[test]
fn test_xyz_parser() {
    use gchemol::io;

    let mut parser = TextParser::default();
    let fname = "tests/multi.xyz";
    let f = File::open(fname).unwrap();

    let mut i = 0;
    parser.parse(f,
                 // parse_plain_xyz,
                 parse_xyz,
                 |p| {
                     i += 1;
                     println!("{}", i);
                 }
    ).unwrap();
}
// xyz:1 ends here
