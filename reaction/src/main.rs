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

// [[file:~/Workspace/Programming/rust-scratch/rust.note::b8ea57f0-b549-4fa0-ac1a-abf83009009e][b8ea57f0-b549-4fa0-ac1a-abf83009009e]]
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

/// read data from file
fn read_from_file(filename: &str){
    // Create a path to the desired file
    let path = Path::new(filename);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let f = match File::open(filename){
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("Couldn't open file {}: {}", display, why.description()),
        Ok(file) => file,
    };

    let mut reader = BufReader::new(f);
    let mut lines_iter = reader.lines().map(|l| l.unwrap());
    let mut timestep = 0 as u32;
    let mut natoms = 0 as u32;
    // representing molecule
    // let mut G = Graph::new_undirected();

    let lbl_timestep = "# Timestep";
    let lbl_natoms   = "# Number of particles";
    'outer: loop {
        // 1. jump to the line containing Timestep
        match lines_iter.nth(0) {
            Some(line) => {
                assert!(line.starts_with(lbl_timestep), line);
                let x = line.chars().as_str().replace(&lbl_timestep, "");
                timestep = x.trim().parse::<u32>().unwrap();
                println!("current timestep: {}", timestep);
            },
            None => {
                break;
            }
        }
        // 2. read in number of atoms
        let line = lines_iter.nth(1).unwrap();
        assert!(line.starts_with(lbl_natoms), line);
        // only necessary for the first time
        if natoms == 0 {
            let x = line.chars().as_str().replace(&lbl_natoms, "");
            natoms = x.trim().parse::<u32>().unwrap();
            println!("number of atoms: {:?}", natoms);
        }
        // 3. read in following 4 commenting lines
        for _ in 0..4 {
            if let Some(line) = lines_iter.next() {
                println!("{}", line);
            } else {
                break; // fail
            }
        }
        // 4. read in bonds lines for each atom
        // construct graph structure
        // let mut node_indices = HashMap::new();
        // for x in 1..(natoms+1) {
        //     let n = G.add_node("X");
        //     node_indices.insert(format!("{}", x), n);
        // }
        // 5. parse current snapshot
        let mut data = vec![];
        for i in 0..natoms {
            match lines_iter.next(){
                Some(line) => {
                    data.push(line);
                },
                None => {
                    panic!("file seems not complete: expected {} lines, acutaully read {:?} lines.", natoms, i)
                }
            }
        }
        let mut G = Graph::new_undirected();
        let mut node_indices = HashMap::new();
        for line in &data {
            // let n = G.add_node(nsymbol);
            let line = &*line;
            let mut attrs = line.split_whitespace();
            let current = attrs.next().unwrap();
            let nsymbol = attrs.next().unwrap();
            let n = G.add_node(nsymbol);
            node_indices.insert(current, n);
        }
        // add bonds
        for line in &data {
            let line = &*line;
            let mut attrs = line.split_whitespace();
            let current = attrs.next().unwrap();
            attrs.next();
            if let Some(nb) = attrs.next() {
                let nb = nb.parse::<u32>().unwrap();
                for _ in 0..nb {
                    let other = attrs.next().unwrap();
                    let n1 = node_indices.get(&current).unwrap();
                    let n2 = node_indices.get(&other).unwrap();
                    G.update_edge(*n1, *n2, 1);
                }
            }
        }
        show_fragments(&G);
        // skip one line
        // line = "#"
        lines_iter.next();
    }
}

/// get file name from command line argument
fn get_filename() -> Result<String, String> {
    use clap::{App, Arg, AppSettings};

    let app = App::new("myapp");

    let matches = App::new("MyApp")
        .version("0.1")
        .author("Wenping Guo <winpng@gmail.com>")
        .about("reaction analysis")
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
        None => Err("bad".to_string())
    }
}

fn main() {
    let filename = get_filename().unwrap();
    read_from_file(filename.as_str());
}
// b8ea57f0-b549-4fa0-ac1a-abf83009009e ends here
