// [[file:~/Workspace/Programming/rust-scratch/rust.note::b08803c2-e9b1-4542-9574-b8c467d527b1][b08803c2-e9b1-4542-9574-b8c467d527b1]]
extern crate base64;

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

fn main() {
    let bytes = base64::decode("aGVsbG8gd29ybGQ=").unwrap();
    println!("{:?}", bytes);

    println!("this is a hello {}", "world");
    println!("{}", 12);

    let mut sum = 0.;
    for i in 0..5 {
        sum += i as f64;
        println!("loop: {}, sum = {}", i, sum);
    }

    let mut v = sqrt(sum);
    println!("{}", v);
}
// b08803c2-e9b1-4542-9574-b8c467d527b1 ends here
