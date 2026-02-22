import argparse
from PythonSelectionSortLorial import selection_sort
from PythonInsertionSortLorial import insertion_sort
from PythonBubbleSortLorial import bubble_sort
from PythonQuickSortLorial import quick_sort
from PythonMergeSortLorial import merge_sort


def main():
    parser = argparse.ArgumentParser(
        description="Run a sorting algorithm from the command line"
    )

    parser.add_argument(
        "algorithm",
        choices=["MERGESORT", "QUICKSORT", "INSERTIONSORT", "BUBBLESORT","SELECTIONSORT"],
        help="Sorting algorithm to run"
    )

    args = parser.parse_args()

    data1 = [100,5,10,1,11]
    data2 = [1,2,-3,4,5]
    data3 = ["k",2,1]
    
    if args.algorithm == "SELECTIONSORT":
        result1 = selection_sort(data1.copy())
        result2 = selection_sort(data2.copy())
        result3 = selection_sort(data3.copy())
    if args.algorithm == "MERGESORT":
        result1 = merge_sort(data1.copy())
        result2 = merge_sort(data2.copy())
        result3 = merge_sort(data3.copy())
    if args.algorithm == "INSERTIONSORT":
        result1 = insertion_sort(data1.copy())
        result2 = insertion_sort(data2.copy())
        result3 = insertion_sort(data3.copy())
    if args.algorithm == "BUBBLESORT":
        result1 = bubble_sort(data1.copy())
        result2 = bubble_sort(data2.copy())
        result3 = bubble_sort(data3.copy())
    if args.algorithm == "QUICKSORT":
        result1 = quick_sort(data1.copy())
        result2 = quick_sort(data2.copy())
        result3 = quick_sort(data3.copy())

    print("Original:", data1)
    if result1 is not None:
        print("Sorted:  ", result1)
    else:
        print("Invalid input")
    
    print("")
    print("Original:", data2)
    if result2 is not None:
        print("Sorted:  ", result2)
    else:
        print("Invalid input")
    
    print("")
    print("Original:", data3)
    if result3 is not None:
        print("Sorted:  ", result3)
    else:
        print("Invalid input")


if __name__ == "__main__":
    main()
