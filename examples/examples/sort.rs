//!
//!
//! ```shell
//! cargo symex --example sort --function FN
//! ```
#![allow(dead_code)]
use symex_lib::{assume, symbolic};

fn selection_sort(arr: &mut [i32]) {
    let n = arr.len();
    for i in 0..n - 1 {
        let mut min = i;
        for j in i + 1..n {
            if arr[j] < arr[min] {
                min = j;
            }
        }
        if min != i {
            let tmp = arr[i];
            arr[i] = arr[min];
            arr[min] = tmp;
        }
    }
}

pub fn check() {
    let mut arr = [0; 5];
    symbolic(&mut arr);
    for element in arr.iter_mut() {
        symbolic(element);
    }

    selection_sort(&mut arr);

    for slice in arr.windows(2) {
        assert!(slice[0] <= slice[1]);
    }
}

fn main() {
    let mut arr = [753, 532, 864, 349, 1425, 34, 65000, 64, 1034, 985];
    selection_sort(&mut arr);

    for slice in arr.windows(2) {
        assume(slice[0] <= slice[1]);
    }
}
