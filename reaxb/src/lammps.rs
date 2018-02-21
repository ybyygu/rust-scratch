// [[file:~/Workspace/Programming/rust-scratch/rust.note::8da0ca06-6b0c-4d1b-8eec-827c7459cf2b][8da0ca06-6b0c-4d1b-8eec-827c7459cf2b]]
use std::fs::File;
use std::error::Error;
use std::io::{self, BufReader, BufWriter};
use std::io::prelude::*;
use std::collections::HashMap;
use std::path::Path;

use petgraph::prelude::*;
use petgraph as pg;

use atoms::{AtomData, TrajectoryFrame, write_as_cif};
use ::Frame;
use graph::fragments_from_atoms;
// 8da0ca06-6b0c-4d1b-8eec-827c7459cf2b ends here

// [[file:~/Workspace/Programming/rust-scratch/rust.note::30c60549-db69-4b1b-b0e5-33904c16a8b9][30c60549-db69-4b1b-b0e5-33904c16a8b9]]
pub fn extract_frame(filename: &str, target_timestep: usize, ciffile: &str) -> Result<(), Box<Error>>
{
    // 1. guess required lammps files from input filename
    let path = Path::new(filename);
    let path_lammps_data = path.with_extension("data");
    let path_lammps_dump = path.with_extension("dump");
    let path_lammps_bonds = path.with_extension("bonds-terse");

    if ! path_lammps_data.is_file() {
        let msg = format!("data file not found: {:}", path_lammps_data.display());
        Err(msg)?;
    }
    if ! path_lammps_bonds.is_file() {
        let msg = format!("bonds file not found: {:}", path_lammps_bonds.display());
        Err(msg)?;
    }

    if ! path_lammps_dump.is_file() {
        let msg = format!("dump file not found: {:}", path_lammps_dump.display());
        Err(msg)?;
    }

    // get positions from dump file
    let mut frame = get_frame_from_lammps_dump_file(&path_lammps_dump, target_timestep)?;

    // get symbols from data file
    frame.symbols = parse_lammps_data_file(&path_lammps_data)?;

    // assign connectivity
    frame.neighbors = get_connectivity_from_terse_bonds_file(&path_lammps_bonds, target_timestep)?;

    let path_ciffile = Path::new(ciffile);
    write_as_cif(frame, &path_ciffile);

    Ok(())
}
// 30c60549-db69-4b1b-b0e5-33904c16a8b9 ends here

// [[file:~/Workspace/Programming/rust-scratch/rust.note::1d1ee6e3-9786-42f5-bc5b-e5542d5e6149][1d1ee6e3-9786-42f5-bc5b-e5542d5e6149]]
pub fn analyze_frames(filename: &str, outfile: &str, maxcols: usize) -> Result<(), Box<Error>>{
    let frames = parse_lammps_files(filename)?;
    write_formated_text(&frames, outfile, maxcols)?;

    Ok(())
}
// 1d1ee6e3-9786-42f5-bc5b-e5542d5e6149 ends here

// [[file:~/Workspace/Programming/rust-scratch/rust.note::2085aabc-b09b-4084-88d1-33699881e5e3][2085aabc-b09b-4084-88d1-33699881e5e3]]
pub fn parse_lammps_files(filename: &str) -> Result<Vec<Frame>, Box<Error>> {
    // 1. guess required lammps files from input filename
    let path = Path::new(filename);
    let path_lammps_data = path.with_extension("data");
    let path_lammps_dump = path.with_extension("dump");
    let path_lammps_bonds = path.with_extension("bonds-terse");

    if ! path_lammps_data.is_file() {
        let msg = format!("data file not found: {:}", path_lammps_data.display());
        Err(msg)?;
    }
    if ! path_lammps_bonds.is_file() {
        let msg = format!("bonds file not found: {:}", path_lammps_bonds.display());
        Err(msg)?;
    }

    // read atom indices and symbols
    let symbols = parse_lammps_data_file(&path_lammps_data)?;

    // assign connectivity
    let frames = parse_terse_bonds_file(&path_lammps_bonds, &symbols);

    frames
}
// 2085aabc-b09b-4084-88d1-33699881e5e3 ends here

// [[file:~/Workspace/Programming/rust-scratch/rust.note::4540ac95-d7d0-42a2-aef3-bcee0abc5586][4540ac95-d7d0-42a2-aef3-bcee0abc5586]]
pub fn write_formated_text(frames: &Vec<Frame>, outfile: &str, max_columns: usize) -> Result<(), Box<Error>>{
    // create output file
    let f = File::create(outfile)?;
    let mut writer = BufWriter::new(f);

    let mut species:HashMap<String, usize> = HashMap::new();
    for frame in frames {
        for (k, v) in &frame.fragments {
            let x = species.entry(k.to_string()).or_insert(0_usize);
            *x += v;
        }
    }

    let mut count_vec: Vec<_> = species.iter().collect();
    count_vec.sort_by_key(|k| k.1);
    count_vec.reverse();

    let vs:Vec<String> = count_vec.iter().map(|x| x.0.to_string()).collect();
    writer.write("Timestep ".as_bytes());

    let mut mc = vs.len();
    if max_columns < mc {
        mc = max_columns;
    }
    let vs = &vs[..mc];
    writer.write(format!("{:}\n", vs.join(" ")).as_bytes());

    for frame in frames {
        let s = format!("{:^width$}", frame.timestep, width="Timestep ".len());
        writer.write(s.as_bytes());
        let mut lst = Vec::new();
        for k in vs.iter() {
            let count = frame.fragments.get(k).unwrap_or(&0_usize);
            lst.push(format!("{:^width$}", count, width=k.len()));
        }
        let s = format!("{}\n", lst.join(" "));
        writer.write(s.as_bytes());
    }

    Ok(())
}
// 4540ac95-d7d0-42a2-aef3-bcee0abc5586 ends here

// [[file:~/Workspace/Programming/rust-scratch/rust.note::1f4bb42e-6c9c-41d1-b9f3-e0908813187a][1f4bb42e-6c9c-41d1-b9f3-e0908813187a]]
/// read data from lammps .data file
fn parse_lammps_data_file(path: &Path) -> Result<HashMap<usize, String>, Box<Error>>
{
    eprintln!("reading data file: {}", path.display());

    let fp = File::open(path)?;
    let mut reader = BufReader::new(fp);
    let mut lines_iter = reader.lines().peekable();

    // sanity check
    if let Some(&Ok(ref firstline)) = lines_iter.peek() {
        if ! firstline.starts_with("LAMMPS data file") {
            let msg = format!("read in a wrong file: {}", firstline);
            Err(msg)?;
        }
    } else {
        let msg = format!("Expect more lines: {}", path.display());
        Err(msg)?;
    }

    // skip the first two lines
    for _ in 0..2 {
        lines_iter.next();
    }

    // 1. read number of total atoms
    // 684  atoms
    let mut natoms = 0;
    if let Some(line) = lines_iter.next() {
        let line = line?;
        assert!(line.contains(" atoms"), format!("cannot find number of atoms: {}", line));
        let mut attrs = line.split_whitespace();
        if let Some(s) = attrs.nth(0) {
            natoms = s.parse().unwrap();
        } else {
            let msg = format!("failed to get natoms: {}", line);
            Err(msg)?;
        }
    } else {
        Err("data file is incomplete: failed to get natoms!")?;
    }

    // 2. read in number of atom types
    let mut natom_types = 0_usize;
    loop {
        if let Some(line) = lines_iter.next() {
            let line = line?;
            if line.ends_with("atom types") {
                if let Some(n) = line.split_whitespace().nth(0) {
                    natom_types = n.parse().unwrap();
                }
                break;
            }
        } else {
            Err("cannot find atom types lines in lammps data file")?;
        }
    }

    // 3. parse atom types
    // NOTE: element symbol is supposed to be after `#`
    //     1  50.941500   # V
    assert!(natom_types > 0);
    let mut mapping_symbols = HashMap::new();
    loop {
        if let Some(line) = lines_iter.next() {
            let line = line?;
            if line.starts_with("Masses") {
                // skip one blank line
                lines_iter.next();
                // mapping: atom_index ==> atom_symbol
                for _ in 0..natom_types {
                    if let Some(line) = lines_iter.next() {
                        let line = line?;
                        let mut attrs = line.split_whitespace();
                        let k = attrs.nth(0).unwrap();
                        let v = attrs.last().unwrap();
                        mapping_symbols.insert(k.to_string(), v.to_string());
                    }
                }
                break;
            }
        } else {
            Err("failed to read Masses section")?;
        }
    }

    // 4. read in atom index and atom type
    assert!(natoms > 0);

    let mut symbols = HashMap::new();
    loop {
        if let Some(line) = lines_iter.next() {
            let line = line?;
            if line.starts_with("Atom") {
                // skip one blank line
                lines_iter.next();
                for _ in 0..natoms {
                    if let Some(line) = lines_iter.next() {
                        let line = line?;
                        let mut attrs = line.split_whitespace();
                        let index = attrs.next().unwrap();
                        let t = attrs.next().unwrap();
                        let index = index.parse().unwrap();
                        let symbol = mapping_symbols.get(t).unwrap().to_string();
                        symbols.insert(index, symbol);
                    } else {
                        Err("Atom records are incomplete.")?;
                    }
                }
                break;
            }
        } else {
            Err("cannot find Atom lines in lammps data file")?;
        }
    }

    Ok(symbols)
}
// 1f4bb42e-6c9c-41d1-b9f3-e0908813187a ends here

// [[file:~/Workspace/Programming/rust-scratch/rust.note::52bb7570-33ac-44ae-950b-c7d67d597e76][52bb7570-33ac-44ae-950b-c7d67d597e76]]
#[test]
#[ignore]
fn test_parse_data_file() {
    let filename = "/home/ybyygu/Incoming/FeC reaxff tests/FeCO/terse-tests/test.data";
    let path = Path::new(&filename);
    let symbols = parse_lammps_data_file(&path);
    println!("{:?}", symbols);
}
// 52bb7570-33ac-44ae-950b-c7d67d597e76 ends here

// [[file:~/Workspace/Programming/rust-scratch/rust.note::daedfe6b-34ed-4dd1-94a2-4e698a00a42c][daedfe6b-34ed-4dd1-94a2-4e698a00a42c]]
fn get_frame_from_lammps_dump_file (path: &Path, target_timestep: usize) -> Result<TrajectoryFrame, Box<Error>>
{
    let fp = File::open(path)?;
    let mut reader = BufReader::new(fp);

    let mut frame = TrajectoryFrame::new();
    let mut natoms = 0_usize;
    let mut timestep = 0_usize;
    let mut buf = String::new();
    loop {
        // 0. sanity check
        buf.clear();
        let nb = reader.read_line(&mut buf)?;
        if nb <= 0 {
            eprintln!("reached the end of the file: {}", path.display());
            break;
        }
        assert!(buf.starts_with("ITEM: TIMESTEP"), format!("Expect the frame header, but: {}", buf));

        // 1. get current timestep
        buf.clear();
        let nb = reader.read_line(&mut buf)?;
        if nb <= 0 {
            Err("Expect more lines: failed to read timestep!")?;
        }
        timestep = buf.trim_right().parse()?;

        // 2. get number of atoms
        for i in 0..2 {
            buf.clear();
            let nb = reader.read_line(&mut buf)?;
            if nb > 0 {
                if i == 1 {
                    natoms = buf.trim_right().parse()?;
                }
            } else {
                Err("Expect more lines: failed to read number of atoms!")?;
            }
        }

        // 3. get lammps box and atoms
        assert!(natoms > 0, buf);
        println!("current timestep = {:?}", timestep);
        if timestep < target_timestep {
            for _ in 0..(natoms + 5) {
                let nb = reader.read_line(&mut buf)?;
                if nb <= 0 {
                    Err("Expect more lines: failed to read lammps box and atoms")?;
                }
            }
        } else if timestep == target_timestep {
            frame.timestep = timestep;
            frame.natoms = natoms;

            buf.clear();
            // 3.1 the lammps box
            for _ in 0..4 {
                let nb = reader.read_line(&mut buf)?;
                if nb <= 0 {
                    Err("Expect more lines: failed to read lammps box!")?;
                }
            }
            let (cell, origin) = get_lammps_dump_box(&buf)?;
            frame.cell = cell;
            frame.cell_origin = origin;

            // 3.2 the atom records
            buf.clear();
            for _ in 0..(natoms+1) {
                let nb = reader.read_line(&mut buf)?;
                if nb <= 0 {
                    Err("Expect more lines: failed to read all atom records!")?;
                }
            }
            let positions = get_lammps_dump_positions(&buf, natoms)?;
            frame.positions = positions;
            // ignore remaining lines
            break;
        } else {
            let msg = format!("Requested timestep {} not found in {}", target_timestep, path.display());
            Err(msg)?;
        }
    }

    Ok(frame)
}
// daedfe6b-34ed-4dd1-94a2-4e698a00a42c ends here

// [[file:~/Workspace/Programming/rust-scratch/rust.note::69313c0d-d969-40b4-a338-ff274407d54d][69313c0d-d969-40b4-a338-ff274407d54d]]
#[test]
#[ignore]
fn test_parse_lammps_dump_file() {
    let filename = "/home/ybyygu/Workspace/Programming/reaction-analysis/tests/FeCO/Fe100_8816_50CO_500_500K.dump";
    let path = Path::new(&filename);

    let frame = get_frame_from_lammps_dump_file(&path, 200_usize).unwrap();
    println!("{:?}", frame.positions);
}
// 69313c0d-d969-40b4-a338-ff274407d54d ends here

// [[file:~/Workspace/Programming/rust-scratch/rust.note::32374d0e-1d81-4ec0-a8b3-0fb7950a625a][32374d0e-1d81-4ec0-a8b3-0fb7950a625a]]
use std::f64;

enum BoxStyle {
    Orthogonal,
    Triclinic,
}

fn get_lammps_dump_box(txt: &str) -> Result<([[f64; 3]; 3], [f64; 3]), Box<Error>>{
    use self::BoxStyle::{Orthogonal, Triclinic};

    let mut lines_iter = txt.lines();

    let mut hi    = [0.0; 3];
    let mut lo    = [0.0; 3];
    let mut tilt  = [0.0; 3];
    let mut style = Orthogonal;

    if let Some(line) = lines_iter.next() {
        if line.starts_with("ITEM: BOX BOUNDS") {
            let attrs = line.split_whitespace();
            style = match attrs.count() {
                6 => Orthogonal,
                9 => Triclinic,
                _ => Err(format!("unexpected box style: {}", &line))?,
            };

            for i in 0..3 {
                if let Some(line) = lines_iter.next() {
                    let mut attrs:Vec<f64> = line.split_whitespace().map(|x| x.parse::<f64>().unwrap()).collect();
                    match style {
                        Orthogonal => {
                            lo[i] = attrs[0];
                            hi[i] = attrs[1];
                        },
                        Triclinic => {
                            lo[i] = attrs[0];
                            hi[i] = attrs[1];
                            tilt[i] = attrs[2];
                        },
                    }
                } else {
                    Err("lammps box is incomplete!")?;
                }
            }
        } else {
            let msg = format!("expect LAMMPS BOX header, but found: {}", &line);
            Err(msg)?;
        }
    } else {
        Err("why")?;
    }

    let mut va = [0.0; 3];
    let mut vb = [0.0; 3];
    let mut vc = [0.0; 3];
    let mut origin = lo;
    match style {
        Orthogonal => {
            va[0] = hi[0] - lo[0];
            vb[1] = hi[1] - lo[1];
            vc[2] = hi[2] - lo[2];
            origin = lo;
        },
        Triclinic  => {
            let xy = tilt[0];
            let xz = tilt[1];
            let yz = tilt[2];

            // x vector
            let xlo = lo[0] - [0.0, xy, xz, xy+xz].iter().fold(f64::MAX, |a, &b| a.min(b));
            let xhi = hi[0] - [0.0, xy, xz, xy+xz].iter().fold(f64::MIN, |a, &b| a.max(b));
            va[0] = xhi - xlo;
            // y vector
            let ylo = lo[1] - [0.0, yz].iter().fold(f64::MAX, |a, &b| a.min(b));
            let yhi = hi[1] - [0.0, yz].iter().fold(f64::MIN, |a, &b| a.max(b));
            vb[0] = xy;
            vb[1] = yhi - ylo;
            // z vector
            let zlo = lo[2];
            let zhi = hi[2];
            vc[0] = xz;
            vc[1] = yz;
            vc[2] = zhi - zlo;
            origin = [xlo, ylo, zlo];
        },
    }

    Ok(([va, vb, vc], origin))
}
// 32374d0e-1d81-4ec0-a8b3-0fb7950a625a ends here

// [[file:~/Workspace/Programming/rust-scratch/rust.note::398e563b-0ad5-4845-a5c3-97c115748e74][398e563b-0ad5-4845-a5c3-97c115748e74]]
#[test]
fn test_lammps_box() {
    let box1 = "ITEM: BOX BOUNDS pp pp pp
-0.195983 11.329
-0.195983 11.329
-0.195983 11.329";

    // results from ovito
    let cell_vector1 = [11.525, 0.0, 0.0];
    let cell_vector2 = [0.0, 11.525, 0.0];
    let cell_vector3 = [0.0, 0.0, 11.525];
    let cell_origin  = [-0.195983, -0.195983, -0.195983];

    let (vts, origin) = get_lammps_dump_box(&box1).unwrap();

    assert_relative_eq!(vts[0][0], cell_vector1[0] as f64, epsilon=1.0e-4);
    assert_relative_eq!(vts[1][1], cell_vector2[1] as f64, epsilon=1.0e-4);
    assert_relative_eq!(vts[2][2], cell_vector3[2] as f64, epsilon=1.0e-4);
    assert_relative_eq!(origin[0], cell_origin[0] as f64, epsilon=1.0e-4);
    assert_relative_eq!(origin[1], cell_origin[1] as f64, epsilon=1.0e-4);
    assert_relative_eq!(origin[2], cell_origin[2] as f64, epsilon=1.0e-4);

    let box2 = "ITEM: BOX BOUNDS xy xz yz pp pp pp
-0.08189 15.3282 -0.045807
0.072939 15.5755 0
0.001924 17.4877 0";

    // results from ovito
    let cell_vector1 = [15.3643, 0.0, 0.0];
    let cell_vector2 = [-0.045807, 15.5026, 0.0];
    let cell_vector3 = [0.0, 0.0, 17.4858];
    let cell_origin  = [-0.036083, 0.072939, 0.001924];

    let (vts, origin) = get_lammps_dump_box(&box2).unwrap();
    assert_relative_eq!(vts[0][0], cell_vector1[0] as f64, epsilon=1.0e-4);
    assert_relative_eq!(vts[1][0], cell_vector2[0] as f64, epsilon=1.0e-4);
    assert_relative_eq!(vts[1][1], cell_vector2[1] as f64, epsilon=1.0e-4);
    assert_relative_eq!(vts[2][2], cell_vector3[2] as f64, epsilon=1.0e-4);
    assert_relative_eq!(origin[0], cell_origin[0] as f64, epsilon=1.0e-4);
    assert_relative_eq!(origin[1], cell_origin[1] as f64, epsilon=1.0e-4);
    assert_relative_eq!(origin[2], cell_origin[2] as f64, epsilon=1.0e-4);

    let box3 = "ITEM: BOX BOUNDS pp pp ff
0.0000000000000000e+00 2.2931000000000001e+01
0.0000000000000000e+00 2.2931000000000001e+01
-1.0000000000000000e+00 5.0497999999999998e+01
";
    let cell_vector1 = [22.931, 0.0, 0.0];
    let cell_vector2 = [0.0, 22.931, 0.0];
    let cell_vector3 = [0.0, 0.0, 51.498];
    let cell_origin = [0.0, 0.0, -1.0];
    let (vts, origin) = get_lammps_dump_box(&box3).unwrap();
    assert_relative_eq!(vts[0][0], cell_vector1[0] as f64, epsilon=1.0e-4);
    assert_relative_eq!(vts[1][1], cell_vector2[1] as f64, epsilon=1.0e-4);
    assert_relative_eq!(vts[2][2], cell_vector3[2] as f64, epsilon=1.0e-4);
    assert_relative_eq!(origin[0], cell_origin[0] as f64, epsilon=1.0e-4);
    assert_relative_eq!(origin[1], cell_origin[1] as f64, epsilon=1.0e-4);
    assert_relative_eq!(origin[2], cell_origin[2] as f64, epsilon=1.0e-4);
}
// 398e563b-0ad5-4845-a5c3-97c115748e74 ends here

// [[file:~/Workspace/Programming/rust-scratch/rust.note::dd0f9789-eb7f-492a-aa0d-24a76d346f76][dd0f9789-eb7f-492a-aa0d-24a76d346f76]]
fn get_lammps_dump_positions(txt: &str, natoms: usize) -> Result<HashMap<usize, [f64; 3]>, Box<Error>>{
    let mut lines_iter = txt.lines();
    if let Some(line) = lines_iter.next() {
        assert!(line.starts_with("ITEM: ATOMS id type x y z"));
    } else {
        Err("failed to read atoms header.")?;
    }

    let mut positions: HashMap<usize, [f64; 3]> = HashMap::new();
    for _ in 0..natoms {
        if let Some(line) = lines_iter.next() {
            let mut attrs:Vec<_> = line.split_whitespace().collect();
            assert!(attrs.len() >= 5, line.to_string());
            let index:usize = attrs[0].parse().unwrap();
            let x:f64 = attrs[2].parse().unwrap();
            let y:f64 = attrs[3].parse().unwrap();
            let z:f64 = attrs[4].parse().unwrap();
            positions.insert(index, [x, y, z]);
        } else {
            Err("atom records are incomplete")?;
        }
    }

    Ok(positions)
}
// dd0f9789-eb7f-492a-aa0d-24a76d346f76 ends here

// [[file:~/Workspace/Programming/rust-scratch/rust.note::afe1d362-438a-4e03-939d-50e9eaac21e1][afe1d362-438a-4e03-939d-50e9eaac21e1]]
#[test]
fn test_parse_dump_positions() {
    let txt = "ITEM: ATOMS id type x y z
1 1 3.77622 3.9054 0.009267
2 1 3.77622 3.9054 2.90503
3 1 1.89072 1.66683 4.37145
4 1 4.97252 1.29462 4.37145
5 2 4.73984 2.09641 0.131493";

    let natoms = 5_usize;
    let positions = get_lammps_dump_positions(&txt, natoms).unwrap();
    assert_relative_eq!(3.77622, &positions[&1_usize][0]);
    assert_relative_eq!(0.131493, &positions[&5_usize][2]);
}
// afe1d362-438a-4e03-939d-50e9eaac21e1 ends here

// [[file:~/Workspace/Programming/rust-scratch/rust.note::8497df51-3eb3-41fc-a87d-c1688c94c29f][8497df51-3eb3-41fc-a87d-c1688c94c29f]]
fn get_connectivity_from_terse_bonds_file(
    path: &Path,
    target_timestep: usize) -> Result<HashMap<usize, Vec<usize>>, Box<Error>>
{
    let fp = File::open(path)?;
    let mut reader = BufReader::new(fp);

    // parse bonds
    let mut neighbors: HashMap<usize, Vec<usize>> = HashMap::new();

    let mut timestep = 0;
    let mut natoms = 0;
    let mut buf = String::new();
    loop {
        // 0. sanity check and get current timestep
        buf.clear();
        let nb = reader.read_line(&mut buf)?;
        if nb <= 0 {
            eprintln!("reached the end of the file: {}", path.display());
            break;
        }

        let label = "# Timestep";
        timestep = get_int_data_from_comment_line(&buf, &label)?;
        eprintln!("{:?}", timestep);

        // 1. get natoms
        buf.clear();
        let nb = reader.read_line(&mut buf)?;
        if nb <= 0 {
            Err("Expect more lines: failed to read number of atoms!")?;
        }
        let label = "# Number of particles";
        natoms = get_int_data_from_comment_line(&buf, &label)?;

        // 2. read atom records
        buf.clear();
        if timestep < target_timestep {
            for _ in 0..natoms {
                let nb = reader.read_line(&mut buf)?;
                if nb <= 0 {
                    Err("Expect more lines: failed to read all atom records!")?;
                }
            }
        } else if timestep == target_timestep {
            for _ in 0..natoms {
                let nb = reader.read_line(&mut buf)?;
                if nb <= 0 {
                    Err("Expect more lines: failed to read all atom records!")?;
                }
            }
            neighbors = get_connectivity_from_terse_bonds_file_frame(&buf, natoms)?;
            // ignore other parts
            break;
        } else {
            let msg = format!("Requested timestep {} not found in {}", target_timestep, path.display());
            Err(msg)?;
        }
    }

    Ok(neighbors)
}
// 8497df51-3eb3-41fc-a87d-c1688c94c29f ends here

// [[file:~/Workspace/Programming/rust-scratch/rust.note::1cb4fcf1-d093-41ce-a011-f88a95c9bf7b][1cb4fcf1-d093-41ce-a011-f88a95c9bf7b]]
fn parse_terse_bonds_file (path: &Path, symbols: &HashMap<usize, String>)
                           -> Result<Vec<Frame>, Box<Error>>
{
    // create file handler and a buffle reader
    let fp = File::open(path)?;
    let mut reader = BufReader::new(fp);
    let mut lines_iter = reader.lines().peekable();

    // parse data
    let mut timestep = 0;
    let mut natoms = 0;
    let mut frames = Vec::new();
    loop {
        if lines_iter.peek().is_none() {
            eprintln!("reached the end of the file: {}", path.display());
            break;
        }

        // process a single frame
        let frame = parse_terse_bonds_file_single_frame(&mut lines_iter, &symbols)?;
        eprintln!("timestep {:}, done.", frame.timestep);
        eprintln!("fragments {:?}", frame.fragments);
        // // for test
        // if frame.timestep > 5000_usize {
        //     break;
        // }
        frames.push(frame);
    }

    Ok(frames)
}
// 1cb4fcf1-d093-41ce-a011-f88a95c9bf7b ends here

// [[file:~/Workspace/Programming/rust-scratch/rust.note::dd3a4020-2ed5-4c62-a6f7-1d80e0fc6198][dd3a4020-2ed5-4c62-a6f7-1d80e0fc6198]]
fn get_connectivity_from_terse_bonds_file_frame(
    txt: &str,
    natoms: usize) -> Result<HashMap<usize, Vec<usize>>, Box<Error>>
{
    let mut neighbors = HashMap::new();

    let mut lines_iter = txt.lines();

    for n in 1..natoms+1 {
        if let Some(line) = lines_iter.next() {
            let (charge, nns) = parse_terse_bonds_file_single_line(&line);
            let mut connected = vec![];
            for x in nns {
                connected.push(x+n);
            }
            neighbors.insert(n, connected);
        } else {
            Err("Atom data is incomplete.")?;
        }
    }

    Ok(neighbors)
}
// dd3a4020-2ed5-4c62-a6f7-1d80e0fc6198 ends here

// [[file:~/Workspace/Programming/rust-scratch/rust.note::b88e850d-3754-49fe-a0a6-cdf0ba8e2169][b88e850d-3754-49fe-a0a6-cdf0ba8e2169]]
#[test]
fn test_get_connectivity_from_terse_bonds_frame() {
    let txt = "0.007 8 9 16 896 1017 1016 905 904 120 121 1 112 128
0.008 128 112 120 121 1 1016 904 896 16 8 1017 905 9
0.009 112 120 128 896 1016 1017 904 905 121 1 8 16 9
0.008 120 121 112 1017 1016 905 904 896 8 9 128 16 1
0.008 896 1017 1016 1 112 904 120 8 905 16 128 121 9";

    let neighbors = get_connectivity_from_terse_bonds_file_frame(&txt, 5_usize).unwrap();
    assert_eq!(neighbors.len(), 5);

    let connected = neighbors.get(&1).unwrap();
    assert_eq!(connected.len(), 13);
    assert_eq!(connected[0], 9);
    assert_eq!(connected[1], 10);
}
// b88e850d-3754-49fe-a0a6-cdf0ba8e2169 ends here

// [[file:~/Workspace/Programming/rust-scratch/rust.note::5f005858-636b-4d77-aca5-e6be1baca10a][5f005858-636b-4d77-aca5-e6be1baca10a]]
fn parse_terse_bonds_file_single_frame<I> (
    lines_iter: &mut I,
    symbols: &HashMap<usize, String>) -> Result<Frame, Box<Error>>
    where I: Iterator<Item=io::Result<String>>,
{
    // 1. read current timestep
    let mut timestep = 0;
    let label = "# Timestep";
    if let Some(line) = lines_iter.next() {
        let line = line?;
        timestep = get_int_data_from_comment_line(&line, &label)?;
    } else {
        Err("Failed to read timestep!")?;
    }

    // 2. read number of atoms
    let mut natoms = 0;
        let label = "# Number of particles";
    if let Some(line) = lines_iter.next() {
        let line = line?;
        natoms = get_int_data_from_comment_line(&line, &label)?;
    } else {
        Err("Failed to read number of atoms!")?;
    }

    // 3. read connectivity for each atom
    let mut atoms = Vec::new();
    assert!(natoms > 0);
    for n in 1..natoms+1 {
        let mut data = AtomData::new();
        if let Some(line) = lines_iter.next() {
            let line = line?;
            let (charge, neighbors) = parse_terse_bonds_file_single_line(&line);
            data.index = n;
            data.charge = charge;
            data.symbol = symbols.get(&data.index).unwrap().to_string();
            for x in neighbors {
                data.neighbors.push(x+n);
            }
            atoms.push(data);
        } else {
            Err("Atom data is incomplete.")?;
        }
    }

    assert!(atoms.len() == natoms);

    // 4. create frame
    let mut frame = Frame::new();
    frame.timestep = timestep;
    let fragments = fragments_from_atoms(&atoms);
    frame.fragments = fragments;

    Ok(frame)
}
// 5f005858-636b-4d77-aca5-e6be1baca10a ends here

// [[file:~/Workspace/Programming/rust-scratch/rust.note::39637608-12b2-4724-ac38-cfc5d4f9c990][39637608-12b2-4724-ac38-cfc5d4f9c990]]
fn parse_terse_bonds_file_single_line(line: &str) -> (f64, Vec<usize>) {
    let mut attrs = line.split_whitespace();
    let first = attrs.nth(0).unwrap();
    let charge:f64 = first.parse().unwrap();
    let neighbors:Vec<usize> = attrs.map(|x| x.parse::<usize>().unwrap()).collect();

    (charge, neighbors)
}
// 39637608-12b2-4724-ac38-cfc5d4f9c990 ends here

// [[file:~/Workspace/Programming/rust-scratch/rust.note::b0d25b51-fbb3-460a-882f-3cdb7c6f6619][b0d25b51-fbb3-460a-882f-3cdb7c6f6619]]
#[test]
fn test_parse_terse_bonds_line() {
    let s = "0.007 8 9 16 896 1017 1016 905 904 120 121 1 112 128";
    let (charge, neighbors) = parse_terse_bonds_file_single_line(&s);
    assert_eq!(0.007, charge);
    assert_eq!(13, neighbors.len());
    assert_eq!(8, neighbors[0]);
}
// b0d25b51-fbb3-460a-882f-3cdb7c6f6619 ends here

// [[file:~/Workspace/Programming/rust-scratch/rust.note::772f2307-4bde-47b4-b839-435dabaf5f1a][772f2307-4bde-47b4-b839-435dabaf5f1a]]
fn get_int_data_from_comment_line(line: &str, prefix: &str) -> Result<usize, String> {
    if line.starts_with(prefix) {
        let s = line[prefix.len()..].trim().parse::<usize>();
        match s {
            Ok(v) => return Ok(v),
            Err(why) => return Err(format!("{:?}", why)),
        }
    } else {
        let msg = format!("Failed to get value {} for current frame: {}", prefix, line);
        Err(msg)
    }
}

#[test]
fn test_get_int_data_from_comment_line() {
    // get timestep
    let r = get_int_data_from_comment_line("# Timestep 10", "# Timestep");
    assert_eq!(r, Ok(10));
    // get number of atoms
    let r = get_int_data_from_comment_line("# Number of particles 684", "# Number of particles");
    assert_eq!(r, Ok(684));

    let r = get_int_data_from_comment_line("# Timestep 0.0", "# Timestep");
    assert!(r.is_err());
    let r = get_int_data_from_comment_line("12 22\n", "# Timestep");
    assert!(r.is_err());
}

// fn get_atom_data_from_line(line: &str) -> Result<(AtomData, &[usize]), String> {
fn get_atom_data_from_line(line: &str) -> Result<AtomData, String> {
    let mut data = AtomData::new();

    let error = format!("Failed to parse atom data from: {}", line);

    // 1. get index
    let mut attrs = line.split_whitespace();
    if let Some(v) = attrs.next() {
        match v.parse::<usize>() {
            Ok(v) => {
                data.index = v;
            },
            Err(why) => {
                return Err(format!("{:?}", why));
            },
        }
    } else {
        return Err(error);
    }

    // 2. get particle type
    if let Some(v) = attrs.next() {
        data.symbol = v.to_string();
    } else {
        return Err("failed to read particle type.".to_string());
    }

    // 3. get number of neighbors
    let mut nneighbors = 0;
    if let Some(v) = attrs.next() {
        match v.parse::<usize>() {
            Ok(v) => {
                nneighbors = v;
            },
            Err(why) => {
                return Err(format!("{:?}", why));
            },
        }
    } else {
        return Err("failed to read number of neighbors.".to_string());
    }

    // 4. get neighbors
    // let mut neighbors = vec![];
    for _ in 0..nneighbors {
        if let Some(v) = attrs.next() {
            match v.parse::<usize>() {
                Ok(v) => {
                    // neighbors.push(v);
                    data.neighbors.push(v);
                },
                Err(why) => {
                    return Err(format!("{:?}", why));
                },
            }
        } else {
            return Err(error);
        }
    }

    Ok(data)
}

#[test]
fn test_get_atom_data_from_line() {
    let line = " 121 3 2 301 28 0         0.978         0.978         1.956         2.000        -0.736 ";
    let r = get_atom_data_from_line(&line);
    assert!(r.is_ok());
    // let (data, _) = r.unwrap();
    let data = r.unwrap();
    assert!(data.index == 121);
    assert!(data.symbol == "3");
}
// 772f2307-4bde-47b4-b839-435dabaf5f1a ends here
