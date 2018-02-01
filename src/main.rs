// [[file:~/Workspace/Programming/rust-scratch/rust.note::f8c09544-ff6f-4589-9c06-d83d9b36e3ab][f8c09544-ff6f-4589-9c06-d83d9b36e3ab]]
// extern crate base64;
#[macro_use]
extern crate nom;
extern crate petgraph;
// f8c09544-ff6f-4589-9c06-d83d9b36e3ab ends here

// [[file:~/Workspace/Programming/rust-scratch/rust.note::b08803c2-e9b1-4542-9574-b8c467d527b1][b08803c2-e9b1-4542-9574-b8c467d527b1]]
fn sqrt(x: f64) -> f64 {
    let mut y:f64 = if x < 5. {5.} else {10.};
    loop {
        y += 1.;
        if y > 15. {
            break;
        }
    }
    y.sqrt()
}

fn test_tuple() -> (i32, i32){
    let t = (1, 2);
    let (a, b) = t;
    let i = 1;
    t
}

fn test_array() {
    let arr: [f64; 100] = [0.1; 100];
    println!("{:?}, {}", arr[10], arr.len());

    let mut arr = [1, 5, 3, 2];
    arr.sort();
    println!("{:?}", arr);
}

fn test_vector() {
    let mut v = vec![1, 2, 0, 5];
    v.insert(0, 13);
    assert_eq!(v, [13, 1, 2, 0, 5]);
    assert_eq!(v[0], 13);
    let v:Vec<i32> = (0..5).collect();
    println!("{:?}", v);
}

fn test_slice() -> i32 {
    let arr = [1, 2, 3, 4];
    let slice = &arr;
    let last = slice.get(3);
    println!("last = {}", last.unwrap());
    // println!("last = {}", arr[5]);
    *last.unwrap()
}

fn test_string () {
    // let mut s = "good to go to do".to_string();
    // s.push('好');
    // let x = s.pop();
    // println!("x={:?}, s={:?}", x.unwrap(), s);
    // println!("{}", "y̆".len());
    // println!("{:?}", "y̆".chars());
    let mut s = "good";
    println!("{:?}", s);
}

fn test_hashmap() {
    use std::collections::HashMap;
    let mut scores = HashMap::new();
    scores.insert("Blue", 10);
    // scores.insert("Blue", 20.); adding float will fail
    println!("{:?}", scores);
}
// b08803c2-e9b1-4542-9574-b8c467d527b1 ends here

// [[file:~/Workspace/Programming/rust-scratch/rust.note::76f3c475-acd4-492d-b4fc-50da7265bfd9][76f3c475-acd4-492d-b4fc-50da7265bfd9]]
fn test_nom(){
    named!(get_greeting<&str,&str>,
           take_s!(2)
    );

    let res = get_greeting("hi there");
    println!("{:?}",res);
}
// 76f3c475-acd4-492d-b4fc-50da7265bfd9 ends here

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
use std::collections::HashMap;

// fn get_reduced_formula<'a, I>(symbols: I) -> Option<&'static str>
//     where I: Iterator<Item=&'static str>,
// {
//     for x in symbols {
//         println!("{:?}", x);
//     }

//     Some("C2H4")
// }

fn get_reduced_formula(symbols: &[&str]) -> String {
    let mut counts = HashMap::new();
    for x in symbols {
        let c = counts.entry(x).or_insert(0);
        *c += 1;
    }

    let mut syms: Vec<String> = Vec::new();
    let mut to_append = String::new();

    for (k, v) in counts {
        let reduced = format!("{}{}", k, v);
        if *k == "C" {
            syms.insert(0, reduced);
        } else if *k == "H" {
            to_append = reduced;
        } else {
            syms.push(reduced);
        }
    }
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

// [[file:~/Workspace/Programming/rust-scratch/rust.note::deb8ea39-2a90-4db1-987b-121d98047d53][deb8ea39-2a90-4db1-987b-121d98047d53]]
fn test_petgraph() {
    use petgraph as pg;
    use std::collections::HashMap;

    let mut G = pg::Graph::new_undirected();
    let aa = [0; 4];
    let bb = [1; 4];
    let x = G.add_node(aa);
    let y = G.add_node(bb);
    let e = G.add_edge(x, y, (1, 1));
    let f = G.add_edge(x, y, (2, 2));
    println!("{:?}", &G[x]);
    println!("{:?}", &G[y]);
    println!("{:?}", e);
    println!("{:?}", &G[e]);
    println!("{:?}", f);
    println!("{:?}", &G[f]);
}

fn test_petgraph_graphmap() {
    use petgraph as pg;
    use petgraph::prelude::*;
    let mut G = UnGraphMap::new();

    let a = Atom{
        index: 1,
        symbol: "H",
    };

    let b = Atom{
        index:2,
        symbol:"H",
    };

    println!("{:?}, {:?}", a, b);
    G.add_node(a);
    G.add_node(b);
    G.add_edge(a, b, 1);
    println!("{:?}", &G);
    println!("{:?}", &G[(a, b)]);
}
// deb8ea39-2a90-4db1-987b-121d98047d53 ends here

// [[file:~/Workspace/Programming/rust-scratch/rust.note::82c97bbd-b1b7-492e-8aa2-31271b45b049][82c97bbd-b1b7-492e-8aa2-31271b45b049]]
fn main() {
    // let bytes = base64::decode("aGVsbG8gd29ybGQ=").unwrap();
    // println!("{:?}", bytes);

    // println!("this is a hello {}", "world");
    // println!("{}", 12);

    // let mut sum = 0.;
    // for i in 0..5 {
    //     sum += i as f64;
    //     println!("loop: {}, sum = {}", i, sum);
    // }

    // let mut v = sqrt(sum);
    // println!("{}", v);

    test_petgraph_graphmap();
}
// 82c97bbd-b1b7-492e-8aa2-31271b45b049 ends here
