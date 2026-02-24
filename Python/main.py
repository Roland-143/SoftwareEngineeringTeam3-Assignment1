"""
main.py (Python)

One entry point to run any of the 5 sorting algorithms.

Usage:
  python main.py <ALGORITHM> [numbers...]

Examples:
  python main.py MERGESORT
  python main.py QUICKSORT 5 3 8 4 2

ALGORITHM options:
  MERGESORT | QUICKSORT | INSERTIONSORT | BUBBLESORT | SELECTIONSORT
"""

from __future__ import annotations

import argparse
import os
import sys
from pathlib import Path
from typing import Callable, List, Optional, Sequence, Tuple, Union

# Make imports work whether you run:
#   (1) cd Python && python main.py ...
#   (2) python Python/main.py ...
THIS_DIR = Path(__file__).resolve().parent
# If this file is placed at repo root, algorithms may live in ./Python.
CANDIDATE_DIRS = [THIS_DIR, THIS_DIR / "Python"]
for d in CANDIDATE_DIRS:
    if d.exists() and d.is_dir():
        sys.path.insert(0, str(d))

from PythonBubbleSortLorial import bubble_sort
from PythonInsertionSortLorial import insertion_sort
from PythonMergeSortLorial import merge_sort
from PythonQuickSortLorial import quick_sort
from PythonSelectionSortLorial import selection_sort


SortFn = Callable[[list], Optional[list]]


ALGORITHMS: dict[str, Tuple[str, SortFn]] = {
    "MERGESORT": ("Merge Sort", merge_sort),
    "QUICKSORT": ("Quick Sort", quick_sort),
    "INSERTIONSORT": ("Insertion Sort", insertion_sort),
    "BUBBLESORT": ("Bubble Sort", bubble_sort),
    "SELECTIONSORT": ("Selection Sort", selection_sort),
}


def _parse_numbers(raw: Sequence[str]) -> Optional[List[int]]:
    """
    Convert a list of tokens to integers.
    Returns None if any token is not an integer.
    """
    out: List[int] = []
    for tok in raw:
        try:
            out.append(int(tok))
        except ValueError:
            return None
    return out


def _prompt_numbers() -> Optional[List[int]]:
    """
    Prompt the user for a list of integers.
    Returns None if the input is empty or invalid.
    """
    user_input = input("Enter integers separated by spaces or commas (or press Enter to skip): ").strip()
    if not user_input:
        return None

    tokens = user_input.replace(",", " ").split()
    return _parse_numbers(tokens)


def _print_case(title: str, original: list, sorted_result: Optional[list]) -> None:
    print(title)
    print("Original:", original)
    if sorted_result is None:
        print("Sorted:  Invalid input")
    else:
        print("Sorted:  ", sorted_result)


def main(argv: Optional[Sequence[str]] = None) -> int:
    parser = argparse.ArgumentParser(description="Run a sorting algorithm (single Python entry point).")

    parser.add_argument(
        "algorithm",
        choices=sorted(ALGORITHMS.keys()),
        help="Sorting algorithm to run",
    )

    # Optional numbers after the algorithm name, e.g.:
    #   python main.py MERGESORT 3 1 2
    parser.add_argument(
        "numbers",
        nargs="*",
        help="Optional integers to sort (space-separated). If omitted, you'll be prompted.",
    )

    args = parser.parse_args(argv)

    algo_key: str = args.algorithm.upper()
    algo_name, sort_fn = ALGORITHMS[algo_key]

    # Match the style of the other language drivers: a few built-in test cases + one user case.
    test1 = [100, 5, 10, 1, 11]
    test2 = [1, 2, -3, 4, 5]
    test3 = ["k", 2, 1]  # intentionally invalid (non-int)

    print("Algorithm:", algo_name)
    print("=" * 40)

    # Always run the built-in tests
    r1 = sort_fn(test1.copy())
    r2 = sort_fn(test2.copy())
    r3 = sort_fn(test3.copy())

    _print_case("Test Array 1:", test1, r1)
    print()
    _print_case("Test Array 2:", test2, r2)
    print()
    _print_case("Test Array 3:", test3, r3)
    print()

    # User test case (either from CLI numbers, or prompt)
    user_numbers: Optional[List[int]]
    if args.numbers:
        user_numbers = _parse_numbers(args.numbers)
        if user_numbers is None:
            print("User Test Array:")
            print("Invalid input (non-integer token found).")
            return 1
    else:
        user_numbers = _prompt_numbers()

    if user_numbers is not None:
        user_result = sort_fn(user_numbers.copy())
        _print_case("User Test Array:", user_numbers, user_result)

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
