//! Bubble-sort example.
//!
//! This example is a degenerate case which is hard to analyze, so expect running times to be high.
//!
//! ```shell
//! cargo symex --example bubble_sort
//! ```
#![allow(dead_code)]
use symex_lib::symbolic;

fn bubble_sort(mut vec: [i32; 3]) -> [i32; 3] {
    loop {
        let mut done = true;
        for i in 0..vec.len() - 1 {
            if vec[i + 1] < vec[i] {
                done = false;
                let temp = vec[i + 1];
                vec[i + 1] = vec[i];
                vec[i] = temp;
            }
        }
        if done {
            return vec;
        }
    }
}

fn main() {
    let mut a = [0; 3];
    symbolic(&mut a);
    bubble_sort(a);
}
