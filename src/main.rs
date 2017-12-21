// [[file:~/Workspace/Programming/rust-scratch/rust.note::b08803c2-e9b1-4542-9574-b8c467d527b1][b08803c2-e9b1-4542-9574-b8c467d527b1]]
// extern crate base64;

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
    print!("{}, {}\n", a, b);
    t
}

fn test_array() {
    let arr: [f64; 100] = [0.1; 100];
    println!("{:?}, {}", arr[10], arr.len());

    let arr = [[1, 5], [2, 4]];
    println!("{:?}", arr[0][1]);
}

fn test_slice() -> i32 {
    let arr = [1, 2, 3, 4];
    let slice = &arr;
    let last = slice.get(3);
    println!("last = {}", last.unwrap());
    // println!("last = {}", arr[5]);
    *last.unwrap()
}

fn test_structs () {
    ;
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
    let v = test_slice();
    print!("{}", v);
}
// b08803c2-e9b1-4542-9574-b8c467d527b1 ends here
