/* 

Name: Mark Pack Jr.
Class ID: 23
Professor: Dr. Dai
Course: CS416 - Software Engineering
Date: February 22nd, 2026

Insertion Sort Implementation in Rust

Commands to run:

rustc InsertionSort.rs
./InsertionSort

* Make sure these commands are run from the /rust directory!

*/

fn insertion_sort(values: &mut [i32]) {
    for i in 1..values.len() {
        let key = values[i];
        let mut j = i;

        while j > 0 && values[j - 1] > key {
            values[j] = values[j - 1];
            j -= 1;
        }

        values[j] = key;
    }
}

fn main() {
    let mut numbers = vec![5, 2, 9, 1, 5, 6];

    println!("Before sorting: {:?}", numbers);

    insertion_sort(&mut numbers);

    println!("After sorting:  {:?}", numbers);
}