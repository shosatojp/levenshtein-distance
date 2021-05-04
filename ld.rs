use ndarray::prelude::*;
use ndarray::Array;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let stra: Vec<char> = args[1].chars().collect();
    let strb: Vec<char> = args[2].chars().collect();
    let mut a = Array::<i64, _>::zeros((stra.len() + 1, strb.len() + 1).f());
    for i in 0..stra.len() + 1 {
        for j in 0..strb.len() + 1 {
            if i == 0 {
                a[[0, j]] = j as i64;
            } else if j == 0 {
                a[[i, 0]] = i as i64;
            } else {
                let a0 = a[[i - 1, j - 1]] + (if stra[i - 1] == strb[j - 1] { 0 } else { 1 });
                let a1 = a[[i - 1, j]] + 1;
                let a2 = a[[i, j - 1]] + 1;
                let min = std::cmp::min(std::cmp::min(a0, a1), a2);
                a[[i, j]] = min;
            }
        }
    }

    println!("{:?}", a);
    println!("d = {}", a[[stra.len(), strb.len()]]);
}
