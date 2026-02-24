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
* We shouldn't really have to run each file individually, you would only
run this file specifically for testing purposes. See main.rs for more information!

*/

// MergeSort.rs (module-style; no main)

use std::time::Instant;

pub fn run(raw_args: &[String]) -> i32 {
    let (quiet, help) = parse_flags(raw_args);
    if help {
        print_help("mergesort");
        return 0;
    }

    let mut numbers = default_numbers();

    if !quiet {
        println!("Merge Sort Demo");
        println!("Before: {:?}", numbers);
    }

    let start = Instant::now();
    merge_sort_in_place(&mut numbers);
    let elapsed = start.elapsed();

    if !quiet {
        println!("After:  {:?}", numbers);
    }
    println!("Execution time: {:.3?}", elapsed);
    0
}

fn merge_sort_in_place(values: &mut [i64]) {
    let n = values.len();
    if n <= 1 {
        return;
    }

    let mid = n / 2;
    merge_sort_in_place(&mut values[..mid]);
    merge_sort_in_place(&mut values[mid..]);

    // Merge into a temp buffer then copy back
    let mut tmp = values.to_vec();
    merge(&values[..mid], &values[mid..], &mut tmp[..]);
    values.copy_from_slice(&tmp);
}

fn merge(left: &[i64], right: &[i64], out: &mut [i64]) {
    let (mut i, mut j, mut k) = (0, 0, 0);

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            out[k] = left[i];
            i += 1;
        } else {
            out[k] = right[j];
            j += 1;
        }
        k += 1;
    }

    while i < left.len() {
        out[k] = left[i];
        i += 1;
        k += 1;
    }
    while j < right.len() {
        out[k] = right[j];
        j += 1;
        k += 1;
    }
}

fn default_numbers() -> Vec<i64> {
    vec![64, 34, 25, 12, 22, 11, 90, -3, 0, 17, 17, 5]
}

fn parse_flags(args: &[String]) -> (bool, bool) {
    let mut quiet = false;
    let mut help = false;
    for a in args {
        match a.as_str() {
            "--quiet" | "-q" => quiet = true,
            "--help" | "-h" => help = true,
            _ => {}
        }
    }
    (quiet, help)
}

fn print_help(name: &str) {
    println!(
        r#"Usage:
  {name} [--quiet|-q] [--help|-h]

Notes:
- Uses a built-in demo list (no custom numbers).
- Prints before/after + execution time."#
    );
}