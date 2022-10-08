//! Example showing using `Vec` works.
//!
//! Only inserting a single element won't trigger a re-allocation of the backing `Vec`.
//! ```shell
//! cargo x0001e --example allocations --function no_realloc
//! ```
//!
//! This will trigger a re-allocation of the backing `Vec`.
//! ```shell
//! cargo x0001e --example allocations --function check_val
//! ```
#![allow(dead_code)]

fn no_realloc() -> u64 {
    let mut z: Vec<u64> = Vec::new();
    z.push(5);
    z[0]
}

fn check_val() -> u64 {
    let mut z = Vec::new();
    z.push(0);
    z.push(1);
    z.push(2);
    z.push(3);
    z.push(4);
    z.push(5);
    z[4]
}

fn main() {
    let mut z = Vec::new();
    z.push(5);
}
