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

// [[file:~/Workspace/Programming/rust-scratch/rust.note::b8ea57f0-b549-4fa0-ac1a-abf83009009e][b8ea57f0-b549-4fa0-ac1a-abf83009009e]]
fn get_edge_from_line(line: &str) -> Vec<(&str, &str)>{
    //  301 4 3 289 308 307 0         1.129         1.232         1.231         3.591         0.083         0.362
    let mut bonds = Vec::new();
    let mut attrs = line.split_whitespace();
    // 1. get the first item

    let current = attrs.nth(0).unwrap();
    attrs.next();
    if let Some(nb) = attrs.next() {
        let nb = nb.parse::<u32>().unwrap();
        for _ in 0..nb {
            let other = attrs.next().unwrap();
            bonds.push((current, other))
        }
    }

    bonds
}

// print all connected components
fn show_fragments(graph: &UnGraph<&str, i32>) {
    let sccs = pg::algo::kosaraju_scc(&graph);
    for x in sccs {
        println!("{:?}", x);
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
    let mut G = Graph::new_undirected();

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
        G.clear();
        // construct graph structure
        let mut node_indices = HashMap::new();
        for x in 1..(natoms+1) {
            let n = G.add_node("X");
            node_indices.insert(format!("{}", x), n);
        }

        for i in 0..natoms {
            match lines_iter.next(){
                Some(line) => {
                    let bonds = get_edge_from_line(&line);
                    for (a1, a2) in bonds {
                        let k1 = node_indices.get(a1).unwrap();
                        let k2 = node_indices.get(a2).unwrap();
                        G.add_edge(*k1, *k2, 1);
                    }
                },
                None => {
                    panic!("file seems not complete: expected {} lines, acutaully read {:?} lines.", natoms, i)
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
