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
* We shouldn't really have to run each file individually, you would only
run this file specifically for testing purposes. See main.rs for more information!

*/


use std::time::Instant;

pub fn run(raw_args: &[String]) -> i32 {
    let (quiet, help) = parse_flags(raw_args);
    if help {
        print_help("selectionsort");
        return 0;
    }

    let mut numbers = default_numbers();

    if !quiet {
        println!("Selection Sort Demo");
        println!("Before: {:?}", numbers);
    }

    let start = Instant::now();
    selection_sort_in_place(&mut numbers);
    let elapsed = start.elapsed();

    if !quiet {
        println!("After:  {:?}", numbers);
    }
    println!("Execution time: {:.3?}", elapsed);
    0
}

fn selection_sort_in_place(values: &mut [i64]) {
    let n = values.len();
    for i in 0..n {
        let mut min_idx = i;
        for j in (i + 1)..n {
            if values[j] < values[min_idx] {
                min_idx = j;
            }
        }
        values.swap(i, min_idx);
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