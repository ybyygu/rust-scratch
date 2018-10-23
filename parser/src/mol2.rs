// base

// [[file:~/Workspace/Programming/rust-scratch/parser/parser.note::*base][base:1]]
use std::collections::HashMap;

use gchemol::{Atom, Bond, Molecule, Lattice, BondKind};

use crate::*;
use nom::IResult;
// base:1 ends here

// atom
// # Sample record
// 3	C3	2.414	0.000	0.000	C.ar	1	BENZENE	0.000

// # Format
// atom_id atom_name x y z atom_type [subst_id [subst_name [charge [status_bit]]]]


// [[file:~/Workspace/Programming/rust-scratch/parser/parser.note::*atom][atom:1]]
named!(read_atoms<&str, Vec<(usize, Atom)>>, preceded!(
    ws!(tag!("@<TRIPOS>ATOM")),
    many0!(atom_record)
));

#[test]
fn test_mol2_get_atoms() {
    let lines = "
@<TRIPOS>ATOM
      1 N           1.3863   -0.2920    0.0135 N.ar    1  UNL1       -0.2603
      2 N          -1.3863    0.2923    0.0068 N.ar    1  UNL1       -0.2603
      3 C           0.9188    0.9708   -0.0188 C.ar    1  UNL1        0.0456
      4 C          -0.4489    1.2590   -0.0221 C.ar    1  UNL1        0.0456
      5 C          -0.9188   -0.9709    0.0073 C.ar    1  UNL1        0.0456
      6 C           0.4489   -1.2591    0.0106 C.ar    1  UNL1        0.0456
      7 H           1.6611    1.7660   -0.0258 H       1  UNL1        0.0845
      8 H          -0.8071    2.2860   -0.0318 H       1  UNL1        0.0845
      9 H           0.8071   -2.2861    0.0273 H       1  UNL1        0.0845
     10 H          -1.6611   -1.7660    0.0214 H       1  UNL1        0.0845

";
    let (_, atoms) = read_atoms(lines).expect("mol2 atoms");
    assert_eq!(10, atoms.len());
}

fn format_atom(a: &Atom) -> String {
    let position = a.position();
    format!("{name:8} {x:-12.5} {y:-12.5} {z:-12.5} {symbol:8} {subst_id:5} {subst_name:8} {charge:-6.4}\n",
            name  = a.label(),
            x = position[0],
            y = position[1],
            z = position[2],
            // FIXME:
            symbol = get_atom_type(a),
            subst_id = 1,
            subst_name = "SUBUNIT",
            charge = 0.0,
    )
}

// parse mol2 atom type. example records:
// C.2, C.3, C.ar
named!(mm_type<&str, (&str, Option<&str>)>, do_parse!(
    symbol: alpha                                    >>
    mtype : opt!(preceded!(tag!("."), alphanumeric)) >>
    (
        (symbol, mtype)
    )
));

#[test]
fn test_mol2_mmtype() {
    let (_, (sym, mtype)) = mm_type("C.ar\n").expect("mol2 atom type");
    assert_eq!("C", sym);
    assert_eq!(Some("ar"), mtype);

    let (_, (sym, mtype)) = mm_type("C.4\n").expect("mol2 atom type 2");
    assert_eq!("C", sym);
    assert_eq!(Some("4"), mtype);

    let (_, (sym, mtype)) = mm_type("C ").expect("mol atom type: missing mm type");
    assert_eq!("C", sym);
    assert_eq!(None, mtype);
}

named!(atom_record<&str, (usize, Atom)>, sp!(do_parse!(
    id        : unsigned_digit         >>     // atom index
    name      : not_space              >>     // atom name
    x         : double                 >>     // cartesian coordinates
    y         : double                 >>
    z         : double                 >>
    emtype    : mm_type                >>
    // substructure and partial charge, which could be omitted
    optional  : opt!(atom_subst_and_charge) >>
                tag!("\n")             >>
    (
        {
            let (e, mtype) = emtype;
            let mut a = Atom::new(e, [x, y, z]);
            a.set_label(name.trim());
            (id, a)
        }
    )
)));

// substructure id and subtructure name
named!(atom_subst_and_charge<&str, (usize, &str, Option<f64>)>, sp!(do_parse!(
    subst_id   : unsigned_digit            >>
    subst_name : not_space                 >>
    charge     : opt!(double)              >>
    status_bit : opt!(alpha)               >>
    (
        (subst_id, subst_name, charge)
    )
)));

#[test]
fn test_formats_mol2_atom() {
    let (r, (_, a)) = atom_record(" 3	C3	2.414	0.000	0.000	C.ar	1	BENZENE	0.000	DICT\n")
        .expect("mol2 full");
    assert_eq!("C", a.symbol());
    let (r, (_, a)) = atom_record(" 3	C3	2.414	0.000	0.000	C.ar	1	BENZENE	0.000\n")
        .expect("mol2 atom: missing status bit");
    assert_eq!("C", a.symbol());
    let (r, (_, a)) = atom_record(" 3	C3	2.414	0.000	0.000	C.ar	1	BENZENE \n")
        .expect("mol2 atom: missing partial charge");
    assert_eq!("C", a.symbol());
    let (r, (_, a)) = atom_record(" 3	C3	2.414	0.000	0.000	C.ar\n")
        .expect("mol2 atom: missing substructure");
    assert_eq!("C", a.symbol());
}

/// simple translation without considering the bonding pattern
/// http://www.sdsc.edu/CCMS/Packages/cambridge/pluto/atom_types.html
/// I just want material studio happy to accept my .mol2 file
fn get_atom_type(atom: &Atom) -> &str {
    match atom.symbol() {
        "C"  => "C.3",
        "P"  => "P.3",
        "Co" => "Co.oh",
        "Ru" => "Ru.oh",
        "O"  => "O.2",
        "N"  => "N.3",
        "S"  => "S.2",
        "Ti" => "Ti.oh",
        "Cr" => "Cr.oh",
        _    => atom.symbol(),
    }
}
// atom:1 ends here

// bond
// # Sample record
// 12	6	12	1
// 	6	5	6	ar
// 5	4	9	am	BACKBONE

// # Format
// bond_id origin_atom_id target_atom_id bond_type [status_bits]


// [[file:~/Workspace/Programming/rust-scratch/parser/parser.note::*bond][bond:1]]
named!(get_bond<&str, (usize, usize, Bond)>, sp!(do_parse!(
        unsigned_digit >>
    n1: unsigned_digit >>
    n2: unsigned_digit >>
    bo: alphanumeric   >>
        read_line      >>
    (
        {
            let bond = match bo
                .to_lowercase()
                .as_ref() {
                "1"  => Bond::single(),
                "2"  => Bond::double(),
                "3"  => Bond::triple(),
                "ar" => Bond::aromatic(),
                "am" => Bond::aromatic(),
                "nc" => Bond::dummy(),
                "wc" => Bond::partial(), // gaussian view use this
                _    => Bond::single()
            };
            (n1, n2, bond)
        }
    )
)));


#[test]
fn test_formats_mol2_bond_record() {
    let (_, (i, j, b)) = get_bond("1	1	2	1 BACKBONE\n")
        .expect("mol2 bond: full");
    assert_eq!(BondKind::Single, b.kind);

    let (_, (i, j, b)) = get_bond("1	1	2	1\n")
        .expect("mol2 bond: missing status bits");
    assert_eq!(BondKind::Single, b.kind);

    let (_, (i, j, b)) = get_bond("1	1	2	ar\n")
        .expect("mol2 bond: aromatic bond type");
    assert_eq!(BondKind::Aromatic, b.kind);
}

// Sample
// ------
// @<TRIPOS>BOND
// 1 1 2 1
// 2 1 3 1
named!(get_bonds_from<&str, Vec<(usize, usize, Bond)>>, do_parse!(
    ws!(tag!("@<TRIPOS>BOND"))  >>
    bonds: many0!(get_bond)    >>
    (
        bonds
    )
));

#[test]
fn test_formats_mol2_bonds() {
    let lines = "@<TRIPOS>BOND
     1    13    11    1
     2    11    12    1
     3     8     4    1
     4     7     3    1
     5     4     3   ar \n\n";

    let (_, x) = get_bonds_from(lines)
        .expect("mol2 bonds");
    assert_eq!(5, x.len());

    let (_, x) = get_bonds_from("@<TRIPOS>BOND\n@<TRIPOS>MOLECULE\n")
        .expect("mol2 bonds: missing bonds");
    assert_eq!(0, x.len());
}

fn format_bond_order(bond: &Bond) -> &str {
    match bond.kind {
        BondKind::Single    => "1",
        BondKind::Double    => "2",
        BondKind::Triple    => "3",
        BondKind::Quadruple => "4",
        BondKind::Aromatic  => "ar",
        BondKind::Partial   => "wk", // gaussian view use this
        BondKind::Dummy     => "nc",
    }
}
// bond:1 ends here

// lattice
// # Format
// @<TRIPOS>CRYSIN
// cell cell cell cell cell cell space_grp setting

// [[file:~/Workspace/Programming/rust-scratch/parser/parser.note::*lattice][lattice:1]]
named!(get_lattice_from<&str, Lattice>, sp!(do_parse!(
                tag!("@<TRIPOS>CRYSIN") >>
                read_until_eol          >>
    a         : double             >>
    b         : double             >>
    c         : double             >>
    alpha     : double             >>
    beta      : double             >>
    gamma     : double             >>
    space_grp : unsigned_digit     >>
    setting   : unsigned_digit     >>
                read_line          >>
    (Lattice::from_params(a, b, c, alpha, beta, gamma))
)));

#[test]
fn test_formats_mol2_crystal() {
    let txt = "@<TRIPOS>CRYSIN
12.312000 4.959000 15.876000 90.000000 99.070000 90.000000 4 1\n";
    let (_, mut x) = get_lattice_from(txt)
        .expect("mol2 crystal");

    assert_eq!([12.312, 4.959, 15.876], x.lengths());
}
// lattice:1 ends here

// molecule
// # Format
// - mol_name
// - num_atoms [num_bonds [num_subst [num_feat [num_sets]]]]
// - mol_type
// - charge_type
// - [status_bits [mol_comment]]


// [[file:~/Workspace/Programming/rust-scratch/parser/parser.note::*molecule][molecule:1]]
named!(get_molecule_from<&str, Molecule>, do_parse!(
                  take_until_and_consume!("@<TRIPOS>MOLECULE") >> sp!(line_ending) >>
    title       : sp!(read_line)                               >>
    counts      : sp!(counts_line)                             >>
    mol_type    : read_line                                    >>
    charge_type : read_line                                    >>
    atoms       : read_atoms                               >>
                  opt!(complete!(take_until!("@<TRIPOS>")))    >>
    bonds       : opt!(complete!(get_bonds_from))              >>
    lattice     : opt!(complete!(get_lattice_from))            >>
                  alt!(complete!(take_until!("@<TRIPOS>MOLECULE")) |
                       take_until!(MAGIC_EOF))                 >>
    (
        {
            let natoms = counts[0];
            if natoms != atoms.len() {
                eprintln!("Inconsistency: expected {} atoms, but found {}", natoms, atoms.len());
            }

            let mut mol = Molecule::new(title);

            // assign atoms
            let mut table = HashMap::new();
            for (i, a) in atoms {
                let n = mol.add_atom(a);
                table.insert(i, n);
            }

            // assign bonds, optionally
            if let Some(bonds) = bonds {
                for (i, j, b) in bonds {
                    let ni = table.get(&i).expect(".mol2 file: look up atom in bond record");
                    let nj = table.get(&j).expect(".mol2 file: look up atom in bond record");
                    mol.add_bond(*ni, *nj, b);
                }
            }

            mol.lattice = lattice;

            mol
        }
    )
));

/// 1 or more unsigned numbers in a line
named!(pub counts_line<&str, Vec<usize>>, terminated!(
    many1!(sp!(unsigned_digit)),
    sp!(line_ending)
));

#[test]
fn test_formats_counts_line() {
    let (_, ns) = counts_line(" 16 14 0 0 0 \n")
        .expect("parser: counts_line");
    assert_eq!(5, ns.len());
}

#[test]
fn test_formats_mol2_molecule() {
    // if missing bonds
    let lines = "# created with PyMOL 2.1.0
@<TRIPOS>MOLECULE
Molecule Name
2
SMALL
USER_CHARGES
@<TRIPOS>ATOM
1	  N1	-1.759	-2.546	0.000	N.3	1	UNK0	0.000
2	  H2	-0.759	-2.575	0.000	 H	1	UNK0	0.000";

    let lines = &format!("{}\n{}", lines, MAGIC_EOF);

    let (_, mol) = get_molecule_from(lines)
        .expect("mol2 format test1");
    assert_eq!(2, mol.natoms());

    // for nonperiodic molecule
    let lines = "# created with PyMOL 2.1.0
@<TRIPOS>MOLECULE
Molecule Name
3
SMALL
USER_CHARGES
@<TRIPOS>ATOM
1	  N1	-1.759	-2.546	0.000	N.3	1	UNK0	0.000
2	  H2	-0.759	-2.575	0.000	 H	1	UNK0	0.000
3	  C3	-2.446	-1.270	0.000	C.3	1	UNK0	0.000
@<TRIPOS>BOND
1 1 2 1
2 1 3 1
@<TRIPOS>SUBSTRUCTURE
1	UNK0	1	GROUP	1 ****	UNK\n";

    let lines = &format!("{}\n{}", lines, MAGIC_EOF);

    let (_, mol) = get_molecule_from(lines).expect("mol2 format test2");
    assert_eq!(3, mol.natoms());
    assert_eq!(2, mol.nbonds());

    // for molecule with periodic crystal
    let lines = "@<TRIPOS>MOLECULE
Molecule Name
12
SMALL
USER_CHARGES
@<TRIPOS>ATOM
1	  N1	-1.759	-2.546	0.000	N.3	1	UNK0	0.000
2	  H2	-0.759	-2.575	0.000	 H	1	UNK0	0.000
3	  C3	-2.446	-1.270	0.000	C.3	1	UNK0	0.000
4	  H4	-3.071	-1.193	-0.890	 H	1	UNK0	0.000
5	  C5	-3.333	-1.124	1.232	C.3	1	UNK0	0.000
6	  C6	-1.456	-0.114	0.000	C.2	1	UNK0	0.000
7	  H7	-2.720	-1.189	2.131	 H	1	UNK0	0.000
8	  H8	-3.836	-0.157	1.206	 H	1	UNK0	0.000
9	  H9	-4.077	-1.920	1.241	 H	1	UNK0	0.000
10	 O10	-0.219	-0.346	0.000	O.2	1	UNK0	0.000
11	 H11	-2.284	-3.396	0.000	 H	1	UNK0	0.000
12	 H12	-1.811	0.895	0.000	 H	1	UNK0	0.000
@<TRIPOS>CRYSIN
18.126000 18.126000 7.567000 90.000000 90.000000 120.000000 191 1
@<TRIPOS>SUBSTRUCTURE
1	UNK0	1	GROUP	1 ****	UNK
\n";

    let lines = &format!("{}\n{}", lines, MAGIC_EOF);
    let (_, mol) = get_molecule_from(lines).expect("mol2 format test3");
    assert_eq!(12, mol.natoms());
    assert_eq!(0, mol.nbonds());
    assert!(mol.lattice.is_some());
}
// molecule:1 ends here
