/* 

Name: Mark Pack Jr.
Class ID: 23
Professor: Dr. Dai
Course: CS416 - Software Engineering
Date: February 22nd, 2026

Quick Sort Implementation in Rust

Commands to run:

rustc QuickSort.rs
./QuickSort

* Make sure these commands are run from the /rust directory!

*/


fn quick_sort(values: &mut [i32]) {
    if values.len() <= 1 {
        return;
    }

    let pivot_index = partition(values);

    quick_sort(&mut values[..pivot_index]);
    quick_sort(&mut values[pivot_index + 1..]);
}

fn partition(values: &mut [i32]) -> usize {
    let len = values.len();
    let pivot = values[len - 1]; // choose last element as pivot

    let mut i = 0;

    for j in 0..len - 1 {
        if values[j] < pivot {
            values.swap(i, j);
            i += 1;
        }
    }

    values.swap(i, len - 1);
    i
}

fn main() {
    let mut numbers = vec![5, 2, 9, 1, 5, 6];

    println!("Before sorting: {:?}", numbers);

    quick_sort(&mut numbers);

    println!("After sorting:  {:?}", numbers);
}