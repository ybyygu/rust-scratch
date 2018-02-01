// [[file:~/Workspace/Programming/rust-scratch/rust.note::b08803c2-e9b1-4542-9574-b8c467d527b1][b08803c2-e9b1-4542-9574-b8c467d527b1]]
// extern crate base64;
#[macro_use]
extern crate nom;
extern crate petgraph;

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

fn test_nom(){
    named!(get_greeting<&str,&str>,
           take_s!(2)
    );

    let res = get_greeting("hi there");
    println!("{:?}",res);
}

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
    G.add_node(("1", "H"));
    G.add_node(("2", "H"));
    G.add_node(("3", "C"));
    G.add_edge(("1", "H"), ("2", "H"), 1);
    println!("{:?}", &G);
}

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
// b08803c2-e9b1-4542-9574-b8c467d527b1 ends here
