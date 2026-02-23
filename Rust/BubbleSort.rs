/* 

Name: Mark Pack Jr.
Class ID: 23
Professor: Dr. Dai
Course: CS416 - Software Engineering
Date: February 22nd, 2026

Bubble Sort Implementation in Rust

Commands to run:

rustc BubbleSort.rs
./BubbleSort

* Make sure these commands are run from the /rust directory!

*/


fn bubble_sort(values: &mut [i32]) {
    let n = values.len();

    for i in 0..n {
        for j in 0..n - 1 - i {
            if values[j] > values[j + 1] {
                values.swap(j, j + 1);
            }
        }
    }
}

fn main() {
    let mut numbers = vec![5, 2, 9, 1, 5, 6];

    println!("Before sorting: {:?}", numbers);

    bubble_sort(&mut numbers);

    println!("After sorting:  {:?}", numbers);
}