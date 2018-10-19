// base

// [[file:~/Workspace/Programming/rust-scratch/parser/parser.note::*base][base:1]]
//! Line originated parser combinators

// for logging
use quicli::prelude::*;

// Indicating the end of stream
pub const MAGIC_EOF: &str = "\n\nxTHIS_IS_THE=MAGIC_END_OF_FILE\n";
// base:1 ends here

// macros

// [[file:~/Workspace/Programming/rust-scratch/parser/parser.note::*macros][macros:1]]
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
// macros:1 ends here

// reexport
// reexport nom combinators


// [[file:~/Workspace/Programming/rust-scratch/parser/parser.note::*reexport][reexport:1]]
pub use nom::{
    self,
    // Recognizes floating point number in a string and returs a f64
    double,
    // Recognizes one or more numerical characters: 0-9
    digit,
    // Recognizes one or more spaces and tabs
    space,
    // Recognizes one or more spaces, tabs, carriage returns and line feeds
    multispace,
    // Recognizes one or more lowercase and uppercase alphabetic characters: a-zA-Z
    alpha,
    alphanumeric,
    // Recognizes an end of line (both '\n' and '\r\n')
    line_ending,
    // Shorter alias
    eol,
    // Everything except eol
    not_line_ending,
};
// reexport:1 ends here

// separators

// [[file:~/Workspace/Programming/rust-scratch/parser/parser.note::*separators][separators:1]]
// Match a blank line containing zero or more whitespace character
named!(pub blank_line<&str, &str>, sp!(nom::line_ending));

/// Anything except whitespace
/// will not consume "\n" character
named!(pub not_space<&str, &str>, is_not!(" \t\r\n"));

/// separator using comma or whitespace
named!(pub comma_or_space<&str, &str>, alt!(
    sp!(tag!(",")) | space
));
// separators:1 ends here

// numbers

// [[file:~/Workspace/Programming/rust-scratch/parser/parser.note::*numbers][numbers:1]]
/// Match one unsigned integer: 123
named!(pub unsigned_digit<&str, usize>, map_res!(
    digit,
    str::parse
));

/// match one signed integer: -1, 0, 1, 2, ...
named!(pub signed_digit<&str, isize>, map_res!(
    recognize!(
        pair!(
            opt!(alt!(char!('+') | char!('-'))),
            digit
        )
    ),
    str::parse
));

#[test]
fn test_parser_signed_digit() {
    let (_, x) = signed_digit("12\n")
        .expect("parser: signed_digit 12");
    assert_eq!(12, x);

    let (_, x) = signed_digit("+12\n")
        .expect("parser: signed_digit +12");
    assert_eq!(12, x);

    let (_, x) = signed_digit("-12\n")
        .expect("parser: signed_digit -12");
    assert_eq!(-12, x);
}
// numbers:1 ends here

// lines

// [[file:~/Workspace/Programming/rust-scratch/parser/parser.note::*lines][lines:1]]
/// Match the remaining line including the eol (end of line) character
named!(pub read_until_eol<&str, &str>, terminated!(
    not_line_ending,
    // alt_complete!(eof | line_ending)
    line_ending
));

#[test]
fn test_parser_read_until_eol() {
    let x = read_until_eol("this is the end\nok\n")
        .expect("parser: read_until_eol");
    let x = read_until_eol("\n")
        .expect("parser: read_until_eol empty line");
}

/// Read m lines from the stream
pub fn read_many_lines(input: &str, m: usize) -> nom::IResult<&str, Vec<&str>> {
    many_m_n!(input, m, m, read_until_eol)
}

#[test]
fn test_parser_read_many_lines() {
    let txt = "12
test
C -11.4286  1.7645  0.0000
C -10.0949  0.9945  0.0000
C -10.0949 -0.5455  0.0000
C -11.4286 -1.3155  0.0000
";
    let (_, lines) = read_many_lines(txt, 3).expect("read_many_lines");
    assert_eq!(3, lines.len());
}
// lines:1 ends here

// coordinates

// [[file:~/Workspace/Programming/rust-scratch/parser/parser.note::*coordinates][coordinates:1]]
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

#[test]
fn test_parser_xyz_array() {
    let (_, x) = xyz_array("-11.4286  1.7645  0.0000 ").unwrap();
    assert_eq!(x[2], 0.0);

    let (_, x) = xyz_array("-11.4286  1.7645  0.0000\n").unwrap();
    assert_eq!(x[2], 0.0);

    let (_, x) = xyz_array("-11.4286\t1.7E-5  0.0000 \n").unwrap();
    assert_eq!(x[2], 0.0);
}
// coordinates:1 ends here

// atom

// [[file:~/Workspace/Programming/rust-scratch/parser/parser.note::*atom][atom:1]]
use gchemol::{Atom, Molecule};

/// Create Atom object from xyz line
/// # Example
/// C -11.4286  1.7645  0.0000
named!(pub read_atom_xyz<&str, Atom>, do_parse!(
    sym      : sp!(alt!(nom::alpha|nom::digit)) >> // element symbol, "1" or "H"
    position : sp!(xyz_array)                   >>
               read_until_eol                   >> // ignore the remaining characters
    (
        Atom::new(sym, position)
    )
));

#[test]
fn test_parser_read_atom() {
    let (_, x) = read_atom_xyz("C -11.4286 -1.3155  0.0000\n").unwrap();
    assert_eq!("C", x.symbol());
    let (_, x) = read_atom_xyz("6 -11.4286 -1.3155  0.0000 \n").unwrap();
    assert_eq!("C", x.symbol());
    assert_eq!(0.0, x.position()[2]);
}

/// Create a list of atoms from many lines in xyz format
/// # Example
/// C -11.4286  1.7645  0.0000
/// C -10.0949  0.9945  0.0000
/// C -10.0949 -0.5455  0.0000
named!(read_atoms_xyz<&str, Vec<Atom>>, many1!(read_atom_xyz));

#[test]
fn test_parser_read_atoms() {
    let txt = "C -11.4286  1.7645  0.0000
C -10.0949  0.9945  0.0000
C -10.0949 -0.5455  0.0000
C -11.4286 -1.3155  0.0000
\n";
    let (_, atoms) = read_atoms_xyz(txt).expect("read_atoms");
    assert_eq!(4, atoms.len());
}
// atom:1 ends here

// molecule

// [[file:~/Workspace/Programming/rust-scratch/parser/parser.note::*molecule][molecule:1]]
/// Create a Molecule object from lines in plain xyz format (coordinates only)
named!(pub read_molecule_pxyz<&str, Molecule>, do_parse!(
    atoms: read_atoms_xyz >>
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

/// Create a Molecule object from lines in xyz format
named!(pub read_molecule_xyz<&str, Molecule>, do_parse!(
    natoms: sp!(terminated!(unsigned_digit, eol)) >>
    title : sp!(read_until_eol)                   >>
    mol   : read_molecule_pxyz                    >>
    (
        {
            mol
        }
    )

));

#[test]
fn test_parser_read_molecule() {
    let txt = "12

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
H -13.7062  1.5395  0.0000\n";

    let txt = format!("{}{}", txt, MAGIC_EOF);
    let (_, mol) = read_molecule_xyz(&txt).unwrap();

    assert_eq!(12, mol.natoms());
}
// molecule:1 ends here

// optinal

// [[file:~/Workspace/Programming/rust-scratch/parser/parser.note::*optinal][optinal:1]]
/// match one or more unsigned numbers separated by whitespace
named!(pub count_many<&str, Vec<usize>>, terminated!(
    many1!(sp!(unsigned_digit)),
    sp!(eol)
));

#[test]
fn test_parser_count_many() {
    let (_, ns) = count_many(" 1 2 3 4 5 \n")
        .expect("parser: count_many");
    assert_eq!(5, ns.len());
}
// optinal:1 ends here
