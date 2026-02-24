use std::io::{self, Write};

#[path = "BubbleSort.rs"]
mod bubblesort;
#[path = "InsertionSort.rs"]
mod insertionsort;
#[path = "MergeSort.rs"]
mod mergesort;
#[path = "QuickSort.rs"]
mod quicksort;
#[path = "SelectionSort.rs"]
mod selectionsort;

fn main() {
    loop {
        println!("\nSorting Demo (Central Program)");
        println!("  1) Bubble Sort");
        println!("  2) Insertion Sort");
        println!("  3) Merge Sort");
        println!("  4) Quick Sort");
        println!("  5) Selection Sort");
        println!("  6) Exit");

        let choice = prompt("Enter choice: ");

        match choice.trim() {
            "1" => run_algo("Bubble Sort", bubblesort::run),
            "2" => run_algo("Insertion Sort", insertionsort::run),
            "3" => run_algo("Merge Sort", mergesort::run),
            "4" => run_algo("Quick Sort", quicksort::run),
            "5" => run_algo("Selection Sort", selectionsort::run),
            "6" => {
                println!("\nExiting...");
                break; // exits the loop, program ends
            }
            _ => {
                println!("Invalid choice. Please select 1–6.");
            }
        }
    }
}

fn run_algo(name: &str, runner: fn(&[String]) -> i32) {
    println!("\n--- {} ---", name);

    let empty: Vec<String> = Vec::new();
    runner(&empty);

    pause();
}

fn prompt(msg: &str) -> String {
    print!("{msg}");
    let _ = io::stdout().flush();

    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);
    input.trim_end().to_string()
}

fn pause() {
    println!("\nPress Enter to continue...");
    let mut dummy = String::new();
    let _ = io::stdin().read_line(&mut dummy);
}