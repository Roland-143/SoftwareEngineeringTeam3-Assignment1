/* 

Name: Mark Pack Jr.
Class ID: 23
Professor: Dr. Dai
Course: CS416 - Software Engineering
Date: February 22nd, 2026

Selection Sort Implementation in Rust

Commands to run:

rustc SelectionSort.rs
./SelectionSort

* Make sure these commands are run from the /rust directory!

*/

fn selection_sort(values: &mut [i32]) {
    let n = values.len();

    for i in 0..n {
        let mut min_index = i;

        for j in i + 1..n {
            if values[j] < values[min_index] {
                min_index = j;
            }
        }

        values.swap(i, min_index);
    }
}

fn main() {
    let mut numbers = vec![5, 2, 9, 1, 5, 6];

    println!("Before sorting: {:?}", numbers);

    selection_sort(&mut numbers);

    println!("After sorting:  {:?}", numbers);
}