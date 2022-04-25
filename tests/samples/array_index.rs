fn get(idx: usize) -> i32 {
    let arr = [0, 1, 2, 3];
    arr[idx]
}

fn get_unchecked(idx: usize) -> i32 {
    let arr = [0, 1, 2, 3];
    unsafe { *arr.get_unchecked(idx) }
}

fn indexing_works() -> i32 {
    get(3)
}

fn main() {
    get(1);
    get_unchecked(1);
    indexing_works();
}
