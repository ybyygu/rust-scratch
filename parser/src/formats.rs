// use

// [[file:~/Workspace/Programming/rust-scratch/parser/parser.note::*use][use:1]]
use std::path::Path;
use std::fs::File;
use std::collections::HashMap;

use quicli::prelude::*;
use crate::*;
use nom::IResult;
use gchemol::{Atom, Bond, Molecule, Lattice, BondKind};
// use:1 ends here

// mods

// [[file:~/Workspace/Programming/rust-scratch/parser/parser.note::*mods][mods:1]]
mod xyz;
mod pdb;
mod mol2;
mod sdf;
mod cif;
// mods:1 ends here

// traits
// Unify behaviors for all chemical file formats


// [[file:~/Workspace/Programming/rust-scratch/parser/parser.note::*traits][traits:1]]
pub trait ChemFileLike {
    /// file type string
    fn ftype(&self) -> &str;

    /// Supported file types in file extension, for example:
    /// [".xyz", ".mol2"]
    fn extensions(&self) -> Vec<&str>;

    /// Determine if file `filename` is parable according to its supported file extensions
    fn parsable(&self, filename: &str) -> bool {
        let filename = filename.to_lowercase();
        for s in self.extensions() {
            if filename.ends_with(&s.to_lowercase()) {
                return true;
            }
        }

        false
    }

    /// Formatted representation of a Molecule
    fn format_molecule(&self, mol: &Molecule) -> Result<String> {
        bail!("unimplemented yet");
    }

    /// format molecules in certain format
    /// file will be read-only if not implemented
    fn format(&self, mols: &[Molecule]) -> Result<String> {
        let mut ms = String::new();
        for mol in mols {
            let m = self.format_molecule(mol)?;
            ms.push_str(&m);
        }

        Ok(ms)
    }

    /// Save multiple molecules into a file
    fn write(&self, filename: &str, mols: &[Molecule]) -> Result<()> {
        // use crate::io::prelude::ToFile;

        let txt = self.format(mols)?;
        // &lines.to_file(filename)?;
        // Ok(())
        unimplemented!()
    }

    /// print a brief description about a chemical file format
    fn describe(&self) {
        println!("filetype: {:?}, possible extensions: {:?}",
                 self.ftype(),
                 self.extensions()
        );
    }

    /// Parse a single molecule from &str using facilities provied by nom crate
    /// file will be write-only if not implemented
    fn parse_molecule<'a>(&self, chunk: &'a str) -> IResult<&'a str, Molecule> {
        unimplemented!()
    }

    /// Default implementation: parse multiple molecules from `filename`
    fn parse<P: AsRef<Path>>(&self, path: P) -> Result<Vec<Molecule>> {
        let path = path.as_ref();
        let fp = File::open(path)
            .map_err(|e| format_err!("failed to open {}: {:?}", path.display(), e))?;

        let parser = TextParser::default();

        let mut mols = vec![];
        parser.parse(fp,
                     // parse a single part
                     |input| {
                         self.parse_molecule(input)
                     },

                     // collect all parts
                     |m| {
                         mols.push(m);
                     }
        )?;

        Ok(mols)
    }
}
// traits:1 ends here
