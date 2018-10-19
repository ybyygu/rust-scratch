// base

// [[file:~/Workspace/Programming/rust-scratch/parser/parser.note::*base][base:1]]
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
            nlines: 1000,
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
                        break;
                    }
                }
            } else {
                // a workaround for nom 4.0 changes: append a magic_eof line to
                // make stream `complete`
                chunk.push_str(MAGIC_EOF);
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
                        debug!("incomplete");
                        break;
                    },

                    // 1.3 found parse errors, just ignore it and continue
                    Err(nom::Err::Error(err)) => {
                        // println!("the context lines: {}", chunk);
                        // eprintln!("found parsing error: {:?}", err);
                        eprintln!("found parsing error");
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

// test

// [[file:~/Workspace/Programming/rust-scratch/parser/parser.note::*test][test:1]]
use gchemol::parser::*;

/// whitespace including one or more spaces or tabs
named!(space_token<&str, &str>, eat_separator!(&b" \t"[..]));
macro_rules! sp (
    ($i:expr, $($args:tt)*) => (
        {
            sep!($i, space_token, $($args)*)
        }
    )
);

//           TOTAL ENERGY            =       -720.18428 EV
named!(total_energy<&str, f64>, do_parse!(
            tag!("TOTAL ENERGY")      >>
            sp!(tag!("="))            >>
    energy: sp!(double_s)             >>
            sp!(tag!("EV"))           >>
    (energy)
));

named!(parse_mopac_output<&str, f64>, do_parse!(
               take_until!(" ** Cite this program as:") >>
               take_until!("TOTAL ENERGY            =") >>
    energy:    total_energy >>
    (
        energy
    )
));


#[test]
fn test_text_parser() {
    use gchemol::io;

    let mut parser = TextParser::default();
    let fname = "/home/ybyygu/Workspace/Programming/gosh/examples/runner/tests/files/mopac/mopac.out";
    let f = File::open(fname).unwrap();

    parser.parse(f, parse_mopac_output, |p| println!("{:#?}", p)).unwrap();
}
// test:1 ends here
