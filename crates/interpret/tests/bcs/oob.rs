fn get(arr: &[i32], idx: usize) -> i32 {
    arr[idx]
}

fn get2(idx: usize) -> i32 {
    let arr = [0, 1, 2];
    get(&arr, idx)
}

fn main() {
    let arr = [0, 1, 2];
    let z = get(&arr, 0);
    let y = get2(0);
}
