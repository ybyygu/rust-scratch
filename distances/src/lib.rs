// read position

// [[file:~/Workspace/Programming/rust-scratch/distances/distances.note::*read%20position][read position:1]]
// ATOM      1  N   SER A  26     285.994 214.551 358.350  1.00 50.00           N
pub fn read_xyz(line: &str) -> Option<[f64; 3]> {
    let mut xyz = [0.0; 3];

    if line.starts_with("ATOM ") {
        let s = &line[28..55];
        let parts: Vec<f64> = s.split_whitespace().map(|v| v.parse().unwrap()).collect();

        debug_assert_eq!(parts.len(), 3, "wrong number of coordinates");
        xyz.copy_from_slice(&parts[..3]);
        return Some(xyz);
    } else {
        return None;
    }
}

#[test]
fn test_read_xyz() {
    let line = "ATOM      1  N   SER A  26     285.994 214.551 358.350  1.00 50.00           N\n";

    let [x, y, z] = read_xyz(line).unwrap();
    assert_eq!(x, 285.994);
    assert_eq!(y, 214.551);
    assert_eq!(z, 358.350);
}
// read position:1 ends here

// read all

// [[file:~/Workspace/Programming/rust-scratch/distances/distances.note::*read%20all][read all:1]]
use quicli::prelude::*;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn read_positions_from_pdb(fname: &str) -> Result<Vec<[f64; 3]>> {
    let f = File::open(fname)?;
    let reader = BufReader::new(f);

    let positions: Vec<_> = reader
        .lines()
        .filter_map(|line| read_xyz(&line.unwrap()))
        .collect();

    info!("got {} points.", positions.len());
    Ok(positions)
}
// read all:1 ends here

// distances

// [[file:~/Workspace/Programming/rust-scratch/distances/distances.note::*distances][distances:1]]
use rayon::prelude::*;

pub fn get_distances(positions: &[[f64; 3]], cutoff: f64) -> Vec<f64> {
    let n = positions.len();

    let dm: Vec<_> = (0..n)
        .into_par_iter()
        .flat_map(|i| {
            (0..i).into_par_iter().map(move |j| {
                distance(&positions[i], &positions[j])
            })
        })
        .filter(|&d| d < cutoff)
        .collect();

    dm
}

#[inline]
fn distance(pi: &[f64; 3], pj: &[f64; 3]) -> f64 {
    let mut dij = 0.0;
    for x in 0..3 {
        dij += (pj[x] - pi[x]).powi(2);
    }

    dij.sqrt()
}

#[test]
fn test_distance_matrix() {
    let pts = vec![[0.0, 0.1, 0.2], [1.0, 1.1, 1.2], [2.0, 2.1, 2.2]];

    let ds = get_distances(&pts, 5.0);
    println!("{:#?}", ds);
}
// distances:1 ends here
