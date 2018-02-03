// [[file:~/Workspace/Programming/rust-scratch/rust.note::68b8f3aa-b3f8-43c0-8b4d-c3165b146535][68b8f3aa-b3f8-43c0-8b4d-c3165b146535]]
extern crate petgraph;
extern crate clap;

use std::fs::File;
use std::error::Error;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::collections::HashMap;
use std::path::Path;

use petgraph::prelude::*;
use petgraph as pg;
// 68b8f3aa-b3f8-43c0-8b4d-c3165b146535 ends here

// [[file:~/Workspace/Programming/rust-scratch/rust.note::1f84ef01-7ddb-4295-8521-c29ad7d7e059][1f84ef01-7ddb-4295-8521-c29ad7d7e059]]
use std::hash::{Hash, Hasher};
use std::cmp::Ordering;

#[derive (Default, Debug, Clone, Copy)]
/// simple atom data structure
pub struct Atom {
    pub index: u64,
    pub symbol: &'static str,
}

impl PartialEq for Atom {
    fn eq(&self, other: &Atom) -> bool {
        self.index == other.index
    }
}

impl Eq for Atom {}

impl Hash for Atom {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.index.hash(state);
    }
}

impl Ord for Atom {
    fn cmp(&self, other: &Atom) -> Ordering {
        self.index.cmp(&other.index)
    }
}

impl PartialOrd for Atom {
    fn partial_cmp(&self, other: &Atom) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[test]
fn test_atom() {
    let a = Atom{
        index: 1,
        symbol: "H",
    };

    let b = Atom {
        index: 2,
        symbol: "H",

    };
    let mut c = Atom {
        index: 1,
        symbol: "H",
    };

    assert!(a != b);
    assert!(a == c);

    assert!(a.index == 1);
    assert!(a.symbol == "H");

    c.symbol = "C";
    assert!(c.symbol == "C");
}
// 1f84ef01-7ddb-4295-8521-c29ad7d7e059 ends here

// [[file:~/Workspace/Programming/rust-scratch/rust.note::27fa4abe-d98c-4dd4-8695-e4b4f807cabc][27fa4abe-d98c-4dd4-8695-e4b4f807cabc]]
// it is better to use generics function,
// but it is really difficult for me now
fn get_reduced_formula(symbols: &[&str]) -> String {
    // 1. count symbols: CCCC ==> C 4
    let mut counts = HashMap::new();
    for x in symbols {
        let c = counts.entry(x).or_insert(0);
        *c += 1;
    }

    let mut syms: Vec<String> = Vec::new();
    let mut to_append = String::new();
    // 2. format the formula
    for (k, v) in counts {
        // 2.1 omit number if the count is 1: C1H4 ==> CH4
        let mut s = String::new();
        if v > 1 {
            s = v.to_string();
        }
        // 2.2 special treatments for C and H
        let reduced = format!("{}{}", k, s);
        if *k == "C" {
            syms.insert(0, reduced);
        } else if *k == "H" {
            to_append = reduced;
        } else {
            syms.push(reduced);
        }
    }
    // 3. final output
    syms.push(to_append);
    let formula = syms.join("");
    formula
}

#[test]
fn test_formula() {
    let symbols   = vec!["C", "H", "C", "H", "H", "H"];
    let formula = get_reduced_formula(&symbols);
    assert!(formula == "C2H4".to_string());
    let symbols   = vec!["C", "H", "C", "H", "H", "O", "H", "O"];
    let formula = get_reduced_formula(&symbols);
    println!("{:?}", formula);
    assert!(formula == "C2O2H4".to_string());
}
// 27fa4abe-d98c-4dd4-8695-e4b4f807cabc ends here

// [[file:~/Workspace/Programming/rust-scratch/rust.note::1f4bb42e-6c9c-41d1-b9f3-e0908813187a][1f4bb42e-6c9c-41d1-b9f3-e0908813187a]]
fn parse_lammps_data_file(file: File) -> Result<String, io::Error>{
    let mut reader = BufReader::new(file);
    let mut lines_iter = reader.lines().peekable();

    // println!("{:?}", lines_iter.peek());
    // println!("{:?}", lines_iter.peek());

    // return Ok("".to_string());

    // skip the first two lines
    for _ in 0..2 {
        lines_iter.next();
    }
    // 1. read number of total atoms
    // 684  atoms
    let mut natoms = 0;
    let nresult = lines_iter.next();
    println!("{:?}", nresult);
    if let Some(line) = nresult {
        // pick out plain string, propagate IO error if any
        let line = line?;
        assert!(line.contains(" atoms"), format!("cannot find number of atoms: {}", line));
        let v: Vec<_> = line.split_whitespace().collect();
        natoms = v[0].parse().unwrap();
    } else {
        eprintln!("{:?}", nresult);
    }

    // 2. read in number of atom types
    let mut natom_types = 0_usize;
    loop {
        if let Some(line) = lines_iter.next() {
            let line = line?;
            println!("{:?}", line);
            if line.ends_with("atom types") {
                if let Some(n) = line.split_whitespace().nth(0) {
                    natom_types = n.parse().unwrap();
                }
                break;
            }
        } else {
            panic!("cannot find atom types lines in lammps data file");
        }
    }

    // 3. parse atom types
    // 3.0 jump to Masses section
    loop {
        if let Some(line) = lines_iter.next() {
            let line = line?;
            if line.starts_with("Masses") {
                break;
            }
        }
    }
    // 3.1 read in atom type maping
    // NOTE: element symbol is supposed to be after `#`
    //     1  50.941500   # V
    // skip one blank line
    assert!(natom_types > 0);
    lines_iter.next();

    // mapping: atom_index ==> atom_symbol
    let mut mapping_symbols = HashMap::new();
    for _ in 0..natom_types {
        if let Some(line) = lines_iter.next() {
            let line = line.unwrap();
            let mut attrs = line.split_whitespace();
            let k = attrs.nth(0).unwrap();
            let v = attrs.last().unwrap();
            mapping_symbols.insert(k.to_string(), v.to_string());
        }
    }
    println!("{:?}", mapping_symbols);

    // 3. jump to Atom section
    loop {
        if let Some(line) = lines_iter.next() {
            let line = line?;
            if line.starts_with("Atom") {
                break;
            }
        } else {
            panic!("cannot find Atom lines in lammps data file");
        }
    }
    // skip one blank line
    lines_iter.next();
    // 4. read in atom index and atom type
    assert!(natoms > 0);
    for _ in 0..natoms {
        if let Some(line) = lines_iter.next() {
            let line = line?;
            // println!("{}", line);
        } else {
            panic!("Atom records are incomplete.");
        }
    }

    Ok("G".to_string())
}
// 1f4bb42e-6c9c-41d1-b9f3-e0908813187a ends here

// [[file:~/Workspace/Programming/rust-scratch/rust.note::a0057eaf-1b2f-4b5d-a261-5e44f026a915][a0057eaf-1b2f-4b5d-a261-5e44f026a915]]
fn parse_lammps_bonds_single_snapshot<I>(lines: &mut I)
    where I: Iterator<Item=io::Result<String>> ,
{
    println!("{:?}", lines.nth(0));
}
// a0057eaf-1b2f-4b5d-a261-5e44f026a915 ends here

// [[file:~/Workspace/Programming/rust-scratch/rust.note::2085aabc-b09b-4084-88d1-33699881e5e3][2085aabc-b09b-4084-88d1-33699881e5e3]]
fn open_lammps_files(filename: &str) -> Result<String, io::Error> {
    // 1. guess required lammps files from input filename
    let path = Path::new(filename);
    let path_lammps_data = path.with_extension("data");
    let path_lammps_bonds = path.with_extension("bonds");
    let path_lammps_dump = path.with_extension("dump");

    assert!(path_lammps_data.is_file());
    assert!(path_lammps_bonds.is_file());

    // Open the path in read-only mode, returns `io::Result<File>`
    let f = File::open(path_lammps_data)?;
    parse_lammps_data_file(f);
    // let f = File:open(path_lammps_bonds)?;
    // parse_lammps_bonds_file(f);

    Ok("Good".to_string())
}

#[test]
fn test_open_file() {
    let path = "/home/ybyygu/Workspace/Projects/structure-prediction/data/e2/789648-d084-401b-a67e-e9628a29ca12/测试文件/V2O5_010_MeOH_rand_nvt_650_20.bonds";
    open_lammps_files(path);
}
// 2085aabc-b09b-4084-88d1-33699881e5e3 ends here

// [[file:~/Workspace/Programming/rust-scratch/rust.note::84783441-0f98-4bd5-87a2-44b54dac4b22][84783441-0f98-4bd5-87a2-44b54dac4b22]]
fn get_edge_from_line(line: String) -> (String, String, Vec<(String, String)>) {
    //  301 4 3 289 308 307 0         1.129         1.232         1.231         3.591         0.083         0.362
    let line = &*line;
    let mut bonds = Vec::new();
    let mut attrs = line.split_whitespace();
    // 1. get the first item

    let current = attrs.nth(0).unwrap();
    let nsymbol = attrs.next().unwrap();

    if let Some(nb) = attrs.next() {
        let nb = nb.parse::<u32>().unwrap();
        for _ in 0..nb {
            let other = attrs.next().unwrap();
            bonds.push((current.to_string(), other.to_string()))
        }
    }
    (current.to_string(), nsymbol.to_string(), bonds)
}

// print all connected components
fn show_fragments(graph: &UnGraph<&str, i32>) {
    let sccs = pg::algo::kosaraju_scc(&graph);
    let mut symbols = vec![];
    for x in sccs {
        symbols.clear();
        for index in x {
            let t = graph[index];
            match t {
                "1" => symbols.push("C"),
                "2" => symbols.push("H"),
                "3" => symbols.push("O"),
                "4" => symbols.push("N"),
                _   => println!("special case: {}", t),
            }
            let formula = get_reduced_formula(&symbols);
            println!("{:?}", formula);
        }
    }
}
// 84783441-0f98-4bd5-87a2-44b54dac4b22 ends here

// [[file:~/Workspace/Programming/rust-scratch/rust.note::b8ea57f0-b549-4fa0-ac1a-abf83009009e][b8ea57f0-b549-4fa0-ac1a-abf83009009e]]
/// get file name from command line argument
fn get_filename() -> Result<String, String> {
    use clap::{App, Arg, AppSettings};

    let app = App::new("myapp");

    let matches = App::new("MyApp")
        .version("0.1")
        .author("Wenping Guo <winpng@gmail.com>")
        .about("lammps/reaxff reaction trajectory analysis")
        .arg(
            Arg::with_name("debug")
                .help("debug switch")
                .long("debug")
                .multiple(true)
                .short("d")
        )
        .arg(
            Arg::with_name("input")
                .help("set input file name")
                .index(1)
        )
        .setting(AppSettings::ArgRequiredElseHelp)
        .get_matches();
    let r = matches.value_of("input");
    match r {
        Some(v) => Ok(v.to_string()),
        None => Err("bad input".to_string())
    }
}

fn main() {
    let filename = get_filename().unwrap();
    // read_from_file(filename.as_str());
    // let r = open_file(filename.as_str());
    // println!("{:?}", r);
}
// b8ea57f0-b549-4fa0-ac1a-abf83009009e ends here
