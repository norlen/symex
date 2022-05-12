// use x0001e::{assume, symbolic};

pub fn bubble_sort(mut vec: [i32; 3]) -> [i32; 3] {
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

fn main() {}
