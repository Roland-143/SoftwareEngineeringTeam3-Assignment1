/* 

Name: Mark Pack Jr.
Class ID: 23
Professor: Dr. Dai
Course: CS416 - Software Engineering
Date: February 22nd, 2026

Merge Sort Implementation in Rust

Commands to run:

rustc MergeSort.rs
./MergeSort

* Make sure these commands are run from the /rust directory!

*/

fn merge_sort(values: &mut [i32]) {
    let len = values.len();

    if len <= 1 {
        return;
    }

    let mid = len / 2;

    // Sort left & right halves
    merge_sort(&mut values[..mid]);
    merge_sort(&mut values[mid..]);

    // Merge the sorted halves
    let mut merged = values.to_vec();

    merge(&values[..mid], &values[mid..], &mut merged[..]);

    values.copy_from_slice(&merged);
}

fn merge(left: &[i32], right: &[i32], result: &mut [i32]) {
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            result[k] = left[i];
            i += 1;
        } else {
            result[k] = right[j];
            j += 1;
        }
        k += 1;
    }

    while i < left.len() {
        result[k] = left[i];
        i += 1;
        k += 1;
    }

    while j < right.len() {
        result[k] = right[j];
        j += 1;
        k += 1;
    }
}

fn main() {
    let mut numbers = vec![5, 2, 9, 1, 5, 6];

    println!("Before sorting: {:?}", numbers);

    merge_sort(&mut numbers);

    println!("After sorting:  {:?}", numbers);
}