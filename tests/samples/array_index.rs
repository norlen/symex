fn get(idx: usize) -> i32 {
    let arr = [0, 1, 2, 3];
    arr[idx]
}

// Cannot currently check this.
fn get_unchecked(idx: usize) -> i32 {
    let arr = [0, 1, 2, 3];
    unsafe { *arr.get_unchecked(idx) }
}

fn main() {
    get(1);
    get_unchecked(1);
}
