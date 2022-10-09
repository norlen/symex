//! Examples showcasing how testing of sorting functions can be done.
//!
//! Note: keep array size low since this example will produce a lot of paths.
//!
//! Check `selection_sort` for errors.
//!
//! ```shell
//! cargo symex --example sort --function check
//! ```
//!
//! Check `selection_sort` for errors and verify that its output is actually sorted, and contains
//! all the elements present in the unsorted array.
//!
//! ```shell
//! cargo symex --example sort --function verify_sort
//! ```
//!
//! Check that `selection_sort` and `quicksort` produce equivalent outputs.
//!
//! ```shell
//! cargo symex --example sort --function functional_equivalence
//! ```
#![allow(dead_code)]
use symex_lib::{assume, symbolic};

const ARRAY_SIZE: usize = 3;

// Check the `selection_sort` function for errors, but don't verify correctness.
fn check() {
    let mut arr = [0; ARRAY_SIZE];
    symbolic(&mut arr);

    selection_sort(&mut arr);
}

// Check the `selection_sort` function for errors and verify its correctness.
fn verify_sort() {
    let mut original = [0; ARRAY_SIZE];
    symbolic(&mut original);

    let mut sorted = original.clone();
    selection_sort(&mut sorted);

    verify(&original, &sorted);
}

// Check both sorts produce equivalent output.
fn functional_equivalence() {
    let mut s1 = [0; ARRAY_SIZE];
    symbolic(&mut s1);

    let mut s2 = s1.clone();
    selection_sort(&mut s1);
    quicksort(&mut s2);
    for i in 0..ARRAY_SIZE {
        assert!(s1[i] == s2[i]);
    }
}

// Selection sort
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
            arr.swap(i, min);
        }
    }
}

// Quicksort and partition functions.
fn quicksort(arr: &mut [i32]) {
    if !arr.is_empty() {
        let p = partition(arr);
        quicksort(&mut arr[..p]);
        quicksort(&mut arr[p + 1..]);
    }
}
fn partition(arr: &mut [i32]) -> usize {
    let pivot = arr[arr.len() - 1];
    let mut i = 0;
    for j in 0..arr.len() - 1 {
        if arr[j] <= pivot {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, arr.len() - 1);
    i
}

// Verify that
// 1. The sort is correct (elements in non-decreasing order)
// 2. All elements in the sorted array are present in the original array.
fn verify<const N: usize>(original: &[i32; N], sorted: &[i32; N]) {
    // Verify sorted is in non-decreasing order.
    for s in sorted.windows(2) {
        assert!(s[0] <= s[1]);
    }

    // Verify sorted elements exist in original.
    let mut found = [false; N];
    for v0 in sorted.iter() {
        let mut exists = false;
        for (i, v1) in original.iter().enumerate() {
            if !found[i] && v0 == v1 {
                exists = true;
                found[i] = true;
                break;
            }
        }
        assert!(exists);
    }
}

fn main() {
    let mut arr = [753, 532, 864, 349, 1425, 34, 65000, 64, 1034, 985];
    selection_sort(&mut arr);

    for slice in arr.windows(2) {
        assume(slice[0] <= slice[1]);
    }
}
