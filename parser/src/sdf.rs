// header

// [[file:~/Workspace/Programming/rust-scratch/parser/parser.note::*header][header:1]]
// MDL SD file format
//
// SD file format reference
// ------------------------
// Ctab block format for V2000
// - http://download.accelrys.com/freeware/ctfile-formats/ctfile-formats.zip
// header:1 ends here

// base

// [[file:~/Workspace/Programming/rust-scratch/parser/parser.note::*base][base:1]]
use gchemol::{Atom, Bond, Molecule, Lattice, BondKind};

use crate::*;
use nom::IResult;
// base:1 ends here

// counts line

// [[file:~/Workspace/Programming/rust-scratch/parser/parser.note::*counts%20line][counts line:1]]
// aaabbblllfffcccsssxxxrrrpppiiimmmvvvvvv
// aaa = number of atoms
// bbb = number of bonds
named!(counts_line<&str, (usize, usize)>, do_parse!(
    natoms: flat_map!(take!(3), sp!(parse_to!(usize))) >> // number of atoms
    nbonds: flat_map!(take!(3), sp!(parse_to!(usize))) >> // number of bonds
            read_line >>  // ignore the remaining
    (
        (natoms, nbonds)
    )
));

#[test]
fn test_sdf_counts_line() {
    let line = " 16 14  0  0  0  0  0  0  0  0999 V2000\n";
    let (_, (na, nb)) = counts_line(line).expect("sdf counts line");
    assert_eq!(16, na);
    assert_eq!(14, nb);
}
// counts line:1 ends here

// atoms

// [[file:~/Workspace/Programming/rust-scratch/parser/parser.note::*atoms][atoms:1]]
// Example input
// -------------
//    -1.2940   -0.5496   -0.0457 C   0  0  0  0  0  0  0  0  0  0  0  0
named!(get_atom_from<&str, Atom>, do_parse!(
    x: flat_map!(take!(10), sp!(parse_to!(f64))) >>
    y: flat_map!(take!(10), sp!(parse_to!(f64))) >>
    z: flat_map!(take!(10), sp!(parse_to!(f64))) >>
    s: take!(3)                                  >> read_line >>
    (
        Atom::new(s.trim(), [x, y, z])
    )
));

// output atom line in .sdf format
fn format_atom(i: usize, a: &Atom) -> String {
    let pos = a.position();
    format!("{x:-10.4} {y:-9.4} {z:-9.4} {sym:3} 0  0  0  0  0  0  0  0  0 {index:2}\n",
        x     = pos[0],
        y     = pos[1],
        z     = pos[2],
        sym   = a.symbol(),
        index = i,
    )
}

#[test]
fn test_sdf_atom() {
    let line = "  -13.5661  206.9157  111.5569 C   0  0  0  0  0  0  0  0  0 12 \n\n";
    let (_, a) = get_atom_from(line).expect("sdf atom");
    let line2 = format_atom(12, &a);
    assert_eq!(line[..60], line2[..60]);
}
// atoms:1 ends here

// bonds

// [[file:~/Workspace/Programming/rust-scratch/parser/parser.note::*bonds][bonds:1]]
//   1  4  1  0  0  0  0
named!(get_bond_from<&str, (usize, usize, Bond)>, do_parse!(
    i: flat_map!(take!(3), sp!(parse_to!(usize))) >>
    j: flat_map!(take!(3), sp!(parse_to!(usize))) >>
    b: flat_map!(take!(3), sp!(parse_to!(usize))) >> read_line >>
    (
        (i, j, Bond::new(b as f64))
    )
));

use std::fmt::Display;
fn format_bond<T: Display>(index1: T, index2: T, bond: &Bond) -> String {
    format!(
        "{index1:>3}{index2:>3}{order:3}  0  0  0 \n",
        index1 = index1,
        index2 = index2,
        order = 1
    )
}

#[test]
fn test_sdf_bond() {
    let line = "  6  7  1  0  0  0 \n";
    let (_, (index1, index2, bond)) = get_bond_from(line).expect("sdf bond");
    let line2 = format_bond(index1, index2, &bond);
    assert_eq!(line[..9], line2[..9]);
}
// bonds:1 ends here

// molecule

// [[file:~/Workspace/Programming/rust-scratch/parser/parser.note::*molecule][molecule:1]]
use std::collections::HashMap;

// will consume the whole file if $$$$ is missing
named!(block_end<&str, Vec<&str>>, call!(read_lines_until, "$$$$"));

#[test]
fn test_sdf_block_end() {
    let lines = "\
M END
> <Name>
MAC-0002655
...
$$$$
";
    let _ = block_end(lines).expect("sdf block end");
}

named!(get_molecule_from<&str, Molecule>, do_parse!(
    title   : read_line             >>
    software: read_line             >>
    comment : read_line             >>
    counts  : counts_line           >>
    atoms   : many1!(get_atom_from) >>
    bonds   : many0!(get_bond_from) >> block_end >>
    (
        {
            let naa = atoms.len();
            let nbb = bonds.len();
            let (na, nb) = counts;
            if na != naa {
                eprintln!("expect {} atoms, but found {}", na, naa);
            }
            if nb != nbb {
                eprintln!("expect {} bonds, but found {}", nb, nbb);
            }

            let mut mol = Molecule::new(title.trim());
            let mut i = 1;
            let mut mapping = HashMap::new();
            for a in atoms {
                let n = mol.add_atom(a);
                mapping.insert(i, n);
                i += 1;
            }

            for (i, j, b) in bonds {
                let ni = mapping[&i];
                let nj = mapping[&j];
                mol.add_bond(ni, nj, b);
            }

            mol
        }
    )
));

#[test]
fn test_sdf_get_molecule() {
    let lines = "\
Configuration number :        7
OpenBabel05141811253D

 16 14  0  0  0  0  0  0  0  0999 V2000
    1.3863   -0.2920    0.0135 N   0  0  0  0  0  0  0  0  0  0  0  0
   -1.3863    0.2923    0.0068 N   0  0  0  0  0  0  0  0  0  0  0  0
    0.9188    0.9708   -0.0188 C   0  0  0  0  0  0  0  0  0  0  0  0
   -0.4489    1.2590   -0.0221 C   0  0  0  0  0  0  0  0  0  0  0  0
   -0.9188   -0.9709    0.0073 C   0  0  0  0  0  0  0  0  0  0  0  0
    0.4489   -1.2591    0.0106 C   0  0  0  0  0  0  0  0  0  0  0  0
    1.6611    1.7660   -0.0258 H   0  0  0  0  0  0  0  0  0  0  0  0
   -0.8071    2.2860   -0.0318 H   0  0  0  0  0  0  0  0  0  0  0  0
    0.8071   -2.2861    0.0273 H   0  0  0  0  0  0  0  0  0  0  0  0
   -1.6611   -1.7660    0.0214 H   0  0  0  0  0  0  0  0  0  0  0  0
    4.1745   -0.5794   -0.3789 O   0  0  0  0  0  0  0  0  0  0  0  0
    3.2019   -0.8118   -0.3826 H   0  0  0  0  0  0  0  0  0  0  0  0
    4.5669   -0.8017    0.5137 H   0  0  0  0  0  0  0  0  0  0  0  0
   -3.7704    0.9637   -1.4942 O   0  0  0  0  0  0  0  0  0  0  0  0
   -3.3519    1.7400   -1.9656 H   0  0  0  0  0  0  0  0  0  0  0  0
   -3.0872    0.5168   -0.9167 H   0  0  0  0  0  0  0  0  0  0  0  0
  2  5  2  0  0  0  0
  3  1  1  0  0  0  0
  4  3  2  0  0  0  0
  4  2  1  0  0  0  0
  5  6  1  0  0  0  0
  5 10  1  0  0  0  0
  6  1  2  0  0  0  0
  6  9  1  0  0  0  0
  7  3  1  0  0  0  0
  8  4  1  0  0  0  0
 11 13  1  0  0  0  0
 12 11  1  0  0  0  0
 14 16  1  0  0  0  0
 15 14  1  0  0  0  0
M  END
$$$$";
    let (_, mol) = get_molecule_from(lines).unwrap();
    assert_eq!(16, mol.natoms());
    assert_eq!(14, mol.nbonds());
}

fn format_molecule(mol: &Molecule) -> String {
    let mut lines = String::new();

    // molecule name
    lines.push_str(&format!("{}\n", mol.name));
    // software
    lines.push_str("gchemol\n");
    // comment
    lines.push_str("\n");
    // counts line
    let line = format!("{natoms:3}{nbonds:3}  0  0  0  0  0  0  0  0999 V2000 \n",
                       natoms=mol.natoms(),
                       nbonds=mol.nbonds());

    lines.push_str(&line);

    for (i, a) in mol.view_atoms() {
        lines.push_str(&format_atom(i, a));
    }

    let bonds = mol.view_bonds();
    for (i, j, b) in bonds {
        lines.push_str(&format_bond(i, j, &b));
    }

    lines.push_str("M  END\n$$$$\n");

    lines
}
// molecule:1 ends here
