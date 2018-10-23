// header

// [[file:~/Workspace/Programming/rust-scratch/parser/parser.note::*header][header:1]]

// header:1 ends here

// base

// [[file:~/Workspace/Programming/rust-scratch/parser/parser.note::*base][base:1]]
use crate::*;

use std::collections::HashMap;
use gchemol::{Atom, Bond, Molecule, Lattice};

use nom::IResult;
// base:1 ends here

// cell loop

// [[file:~/Workspace/Programming/rust-scratch/parser/parser.note::*cell%20loop][cell loop:1]]
use nom::recognize_float;

/// Recognizes a float point number with uncertainty brackets
/// # Example
/// 10.154(2)
named!(double_cif<&str, f64>, do_parse!(
    v: recognize_float                                 >>
    o: opt!(delimited!(char!('('), digit, char!(')'))) >>

    (
        {
            let s = if let Some(o) = o {
                v.to_owned() + o
            } else {
                v.to_owned()
            };

            s.parse().expect("cif uncertainty number")
        }
    )
));

#[test]
fn test_cif_float_number() {
    let (_, v) = double_cif("0.3916\n").expect("cif float1");
    assert_eq!(v, 0.3916);
    let (_, v) = double_cif("0.391(6)\n").expect("cif float2");
    assert_eq!(v, 0.3916);
}

/// Recognizes a float value with a preceeding tag
fn tagged_f64<'a>(input: &'a str, tag: &'a str) -> IResult<&'a str, f64> {
    preceded!(input, sp!(tag!(tag)), sp!(double_cif))
}

#[test]
fn test_tagged_f64() {
    let (_, v) = tagged_f64(" abc 4.1 \n", "abc").expect("cif tagged f64");
    assert_eq!(4.1, v);
}

named!(cell_params<&str, (f64, f64, f64, f64, f64, f64)>, permutation!(
    ws!(call!(tagged_f64, "_cell_length_a")),
    ws!(call!(tagged_f64, "_cell_length_b")),
    ws!(call!(tagged_f64, "_cell_length_c")),
    ws!(call!(tagged_f64, "_cell_angle_alpha")),
    ws!(call!(tagged_f64, "_cell_angle_beta")),
    ws!(call!(tagged_f64, "_cell_angle_gamma"))
));


/// Read crystal cell
named!(read_cell<&str, Lattice>, do_parse!(
    params: cell_params >>
    (
        Lattice::from_params(
            params.0,
            params.1,
            params.2,
            params.3,
            params.4,
            params.5,
        )
    )
));

#[test]
fn test_cif_cell_loop() {
    // Allow data in random order and with blank line
    let lines = "_cell_length_a                    18.094(0)
_cell_length_c                    7.5240

_cell_length_b                    20.5160
_cell_angle_alpha                 90.0000
_cell_angle_beta                  90.0000
_cell_angle_gamma                 90.0000
loop_
";

    let (_, param) = cell_params(lines).expect("cif cell");
    assert_eq!(param.1, 20.5160);
}
// cell loop:1 ends here

// atom sites loop
// # Example
// loop_
// _atom_site_label
// _atom_site_type_symbol
// _atom_site_fract_x
// _atom_site_fract_y
// _atom_site_fract_z
// Cu1 Cu 0.20761(4) 0.65105(3) 0.41306(4)
// O1 O 0.4125(2) 0.6749(2) 0.5651(3)
// O2 O 0.1662(2) 0.4540(2) 0.3821(3)
// O3 O 0.4141(4) 0.3916(3) 0.6360(4)
// N1 N 0.2759(3) 0.8588(2) 0.4883(3)
// ...


// [[file:~/Workspace/Programming/rust-scratch/parser/parser.note::*atom%20sites%20loop][atom sites loop:1]]
named!(atom_site_header<&str, &str>, preceded!(
    tag!("_atom_site_"),
    not_space
));

// ["label", "type_symbol", "fract_x", "fract_y", "fract_z", "U_iso_or_equiv", "adp_type", "occupancy"]
named!(atom_site_headers<&str, Vec<&str>>, do_parse!(
    headers: many1!(ws!(atom_site_header)) >>
    (
        headers
    )
));

#[test]
fn test_cif_site_headers() {
    let lines = "_atom_site_label
_atom_site_type_symbol
_atom_site_fract_x
_atom_site_fract_y

_atom_site_fract_z
_atom_site_U_iso_or_equiv
_atom_site_adp_type
_atom_site_occupancy
Si1    Si    0.30070   0.07240   0.04120   0.00000  Uiso   1.00 \n";

    let (_, h) = atom_site_headers(lines).expect("cif atom site headers");
    assert_eq!(h.len(), 8);
}

/// Read atoms from cif _atom_site loop
fn read_atoms<'a>(input: &'a str) -> IResult<&'a str, Vec<Atom>> {
    let (rest, headers) = atom_site_headers(input)?;
    // TODO: early return using return_error! macro
    if headers.len() <= 4 {
        println!("{:?}", rest);
        eprintln!("cif formats: not enough columns in atom site loop");
    }

    // column header loopup table
    // Example
    // -------
    //   0        1         2       3      4            5          6         7
    // label type_symbol fract_x fract_y fract_z U_iso_or_equiv adp_type occupancy
    let table: HashMap<_, _> = HashMap::from(headers.iter().zip(0..).collect());
    let ifx = *table.get(&"fract_x").expect("fract x col");
    let ify = *table.get(&"fract_y").expect("fract y col");
    let ifz = *table.get(&"fract_z").expect("fract z col");
    // column index to atom label
    let ilbl = *table.get(&"label").expect("atom label col");
    // TODO: column index to element symbol, which is optional
    let isym = *table.get(&"type_symbol").expect("atom symbol col");

    do_parse!(rest,
        rows: many1!(
            terminated!(
                count!(sp!(not_space), headers.len()),
                sp!(line_ending)
            )
        ) >>
        (
            {
                let mut atoms = vec![];
                for row in rows {
                    let fx: f64 = row[ifx].parse().map_err(|err| {
                        nom::Err::Failure(
                            nom::Context::Code(row[ifx], nom::ErrorKind::Custom(28))
                        )}
                    )?;

                    let fy: f64 = row[ify].parse().map_err(|err| {
                        nom::Err::Failure(
                            nom::Context::Code(row[ify], nom::ErrorKind::Custom(28))
                        )}
                    )?;

                    let fz: f64 = row[ifz].parse().map_err(|err| {
                        nom::Err::Failure(
                            nom::Context::Code(row[ifz], nom::ErrorKind::Custom(28))
                        )}
                    )?;

                    let lbl = row[ilbl];
                    let sym = row[isym];
                    // TODO: assign atom label
                    let a = Atom::build()
                        .symbol(sym)
                        .position(fx, fy, fz)
                        .finish();
                    atoms.push(a);
                }

                atoms
            }
        )
    )
}

#[test]
fn test_cif_atoms() {
    let lines = "_atom_site_label
_atom_site_type_symbol
_atom_site_fract_x
_atom_site_fract_y
_atom_site_fract_z
_atom_site_U_iso_or_equiv
_atom_site_adp_type
_atom_site_occupancy
Si1    Si    0.30070   0.07240   0.04120   0.00000  Uiso   1.00
Si2    Si    0.30370   0.30880   0.04610   0.00000  Uiso   1.00
O3     O     0.12430   0.41700   0.42870   0.00000  Uiso   1.00
O4     O     0.12260   0.19540   0.42540   0.00000  Uiso   1.00
O5     O     0.23620   0.12240   0.98650   0.00000  Uiso   1.00
Si6    Si    0.80070   0.57240   0.04120   0.00000  Uiso   1.00
Si7    Si    0.80370   0.80880   0.04610   0.00000  Uiso   1.00
O8     O     0.62430   0.91700   0.42870   0.00000  Uiso   1.00
O9     O     0.62260   0.69540   0.42540   0.00000  Uiso   1.00
O10    O     0.73620   0.62240   0.98650   0.00000  Uiso   1.00
Si11   Si    0.69930   0.92760   0.54120   0.00000  Uiso   1.00
Si12   Si    0.69630   0.69120   0.54610   0.00000  Uiso   1.00
\n";
    let (r, v) = read_atoms(lines)
        .expect("cif atom site loop");
    assert_eq!(12, v.len());
}
// atom sites loop:1 ends here

// bond loop
// # Example
// loop_
// _geom_bond_atom_site_label_1
// _geom_bond_atom_site_label_2
// _geom_bond_distance
// _geom_bond_site_symmetry_2
// _ccdc_geom_bond_type
// Si1    O140    1.629   .     S
// Si1    O128    1.624   .     S
// Si1    O5      1.607   1_554 S
// Si1    O18     1.614   1_554 S
// Si2    O86     1.587   .     S
// ...

// [[file:~/Workspace/Programming/rust-scratch/parser/parser.note::*bond%20loop][bond loop:1]]
named!(geom_bond_header<&str, &str>, preceded!(
    alt!(
        tag!("_geom_bond_") |
        tag!("_ccdc_geom_bond_")
    ),
    not_space
));

named!(geom_bond_headers<&str, Vec<&str>>, preceded!(
    ws!(tag!("loop_")),
    many1!(ws!(geom_bond_header))
));

#[test]
fn test_cif_bond_header() {
    let txt = "loop_
_geom_bond_atom_site_label_1
_geom_bond_atom_site_label_2
_geom_bond_distance
_geom_bond_site_symmetry_2
_ccdc_geom_bond_type
# END
";

    let (_, x) = geom_bond_headers(txt).expect("cif bond headers");
    assert_eq!(5, x.len());
}
// bond loop:1 ends here

// molecule

// [[file:~/Workspace/Programming/rust-scratch/parser/parser.note::*molecule][molecule:1]]
// The first line
named!(cif_title<&str, &str>, preceded!(
    sp!(tag!("data_")),
    not_space
));

/// Create Molecule object from cif stream
named!(read_molecule<&str, Molecule>, do_parse!(
    name : cif_title              >>
           take_until!("\n_cell") >>
    lat  : ws!(read_cell)         >>
           take_until!("\n_atom") >>
    atoms: ws!(read_atoms)        >>
    (
        {
            // TODO: add bonds
            let mut mol = Molecule::new(name);

            for mut a in atoms {
                let p = lat.to_cart(a.position());
                a.set_position(p);
                mol.add_atom(a);
            }

            mol.set_lattice(lat);

            mol
        }
    )
));

#[test]
fn test_cif_molecule() {
    let lines = "data_LTL

# CIF taken from the IZA-SC Database of Zeolite Structures
# Ch. Baerlocher and L.B. McCusker
# Database of Zeolite Structures: http://www.iza-structure.org/databases/

_cell_length_a                  18.12600
_cell_length_b                  18.12600
_cell_length_c                   7.56700
_cell_angle_alpha               90.00000
_cell_angle_beta                90.00000
_cell_angle_gamma              120.00000

_symmetry_space_group_name_H-M     'P 6/m m m'
_symmetry_Int_Tables_number         191
_symmetry_cell_setting             hexagonal

loop_
_symmetry_equiv_pos_as_xyz
'+x,+y,+z'
'-y,+x-y,+z'
'-x+y,-x,+z'
'-x,-y,+z'
'+y,-x+y,+z'
'+x-y,+x,+z'
'-y,-x,+z'
'-x+y,+y,+z'
'+x,+x-y,+z'
'+y,+x,+z'
'+x-y,-y,+z'
'-x,-x+y,+z'
'-x,-y,-z'
'+y,-x+y,-z'
'+x-y,+x,-z'
'+x,+y,-z'
'-y,+x-y,-z'
'-x+y,-x,-z'
'+y,+x,-z'
'+x-y,-y,-z'
'-x,-x+y,-z'
'-y,-x,-z'
'-x+y,+y,-z'
'+x,+x-y,-z'

loop_
_atom_site_label
_atom_site_type_symbol
_atom_site_fract_x
_atom_site_fract_y
_atom_site_fract_z
    O1    O     0.2645    0.5289    0.2231
    O2    O     0.1099    0.4162    0.3263
    O3    O     0.1484    0.5742    0.2620
    O4    O     0.1365    0.4736    0.0000
    O5    O     0.0000    0.2797    0.5000
    O6    O     0.1628    0.3256    0.5000
    T1    Si    0.1648    0.4982    0.2030
    T2    Si    0.0959    0.3594    0.5000

#END";

    let (_, x) = read_molecule(lines).expect("cif molecule");
    assert_eq!(8, x.natoms());
}
// molecule:1 ends here
