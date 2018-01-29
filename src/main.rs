// [[file:~/Workspace/Programming/rust-scratch/rust.note::b08803c2-e9b1-4542-9574-b8c467d527b1][b08803c2-e9b1-4542-9574-b8c467d527b1]]
// extern crate base64;
#[macro_use]
extern crate nom;

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

fn main() {
    // let bytes = base64::decode("aGVsbG8gd29ybGQ=").unwrap();
    // println!("{:?}", bytes);

    println!("this is a hello {}", "world");
    println!("{}", 12);

    let mut sum = 0.;
    for i in 0..5 {
        sum += i as f64;
        println!("loop: {}, sum = {}", i, sum);
    }

    let mut v = sqrt(sum);
    println!("{}", v);
    // test_hashmap();
    test_nom();
}
// b08803c2-e9b1-4542-9574-b8c467d527b1 ends here
