// base

// [[file:~/Workspace/Programming/rust-scratch/parser/parser.note::*base][base:1]]
use quicli::prelude::*;

use nom::{
    double,
    space,
};

// Indicating the end of stream
pub const MAGIC_EOF: &str = "\nxTHIS_IS_THE=MAGIC_END_OF_FILE\n";

named!(pub eof<&str, &str>, tag!(MAGIC_EOF));

named!(pub space_token<&str, &str>, eat_separator!(&b" \t"[..]));

#[macro_export]
macro_rules! sp (
    ($i:expr, $($args:tt)*) => (
        {
            sep!($i, space_token, $($args)*)
        }
    )
);

/// Consume three float numbers separated by one or more spaces
/// Return position array
named!(pub xyz_array<&str, [f64; 3]>, do_parse!(
    x: double   >>
       space    >>
    y: double   >>
       space    >>
    z: double   >>
    (
        [x, y, z]
    )
));
// base:1 ends here

// atom/molecule

// [[file:~/Workspace/Programming/rust-scratch/parser/parser.note::*atom/molecule][atom/molecule:1]]
use nom;
use gchemol::{Atom, Molecule};

use nom::{
    digit,
    eol,
};

/// match one unsigned integer
named!(pub unsigned_digit<&str, usize>, map_res!(
    digit,
    str::parse
));

// sections are separated by a blank line
named!(pub blank_line<&str, &str>, sp!(nom::line_ending));

/// read the remaining line including the eol character
named!(pub read_until_eol<&str, &str>, terminated!(
    nom::not_line_ending,
    nom::eol
));

named!(get_atom_from<&str, Atom>, do_parse!(
    sym      : sp!(alt!(nom::alpha|nom::digit)) >> // element symbol, "1" or "H"
    position : sp!(xyz_array)                   >>
               read_until_eol                   >> // ignore the remaining characters
    (
        Atom::new(sym, position)
    )
));

named!(get_molecule_pxyz<&str, Molecule>, do_parse!(
    atoms: many1!(get_atom_from)          >>
    (
        {
            let mut mol = Molecule::new("plain xyz");
            for a in atoms {
                mol.add_atom(a);
            }
            mol
        }
    )
));

named!(get_molecule_xyz<&str, Molecule>, do_parse!(
    natoms: sp!(terminated!(unsigned_digit, eol)) >>
    title : sp!(read_until_eol)                   >>
    mol   : get_molecule_pxyz                     >>
    (
        {
            mol
        }
    )

));

#[test]
fn test_formats_xyz_molecule() {
    let txt = "12
test
C -11.4286  1.7645  0.0000
C -10.0949  0.9945  0.0000
C -10.0949 -0.5455  0.0000
C -11.4286 -1.3155  0.0000
C -12.7623 -0.5455  0.0000
C -12.7623  0.9945  0.0000
H -11.4286  2.8545  0.0000
H -9.1509  1.5395  0.0000
H -9.1509 -1.0905  0.0000
H -11.4286 -2.4055  0.0000
H -13.7062 -1.0905  0.0000
H -13.7062  1.5395  0.0000 ";

    let txt = format!("{}{}", txt, MAGIC_EOF);
    println!("{:}", txt);

    let (_, mol) = get_molecule_xyz(&txt).unwrap();

    assert_eq!(12, mol.natoms());
}
// atom/molecule:1 ends here
