fn get(arr: &[i32], idx: usize) -> i32 {
    arr[idx]
}

fn main() {
    let arr = [0, 1, 2];
    let z = get(&arr, 0);
}
