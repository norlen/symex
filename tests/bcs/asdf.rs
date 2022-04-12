fn get(arr: &mut [i32], idx: usize) -> i32 {
    arr[idx] = 5;
    println!("{}", arr[idx]);
    arr[idx]
}

fn get2(idx: usize) -> i32 {
    let mut arr = [0, 1, 2];
    get(&mut arr, idx)
}

fn main() {
    let mut arr = [0, 1, 2];
    let z = get(&mut arr, 0);
    let y = get2(0);
}
