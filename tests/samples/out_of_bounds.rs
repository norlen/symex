fn out_of_bounds(idx: usize) -> i32 {
    let arr = [0, 1, 2, 3];
    arr[idx]
}

fn out_of_bounds_unchecked(idx: usize) -> i32 {
    let arr = [0, 1, 2, 3];
    unsafe { *arr.get_unchecked(idx) }
}

fn main() {
    out_of_bounds(1);
    out_of_bounds_unchecked(1);
}
