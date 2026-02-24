import argparse
from PythonQuickSortLorial import quick_sort


def main():
    parser = argparse.ArgumentParser(
        description="Run a sorting algorithm from the command line"
    )

    parser.add_argument(
        "algorithm",
        choices=["QUICKSORT"],
        help="Sorting algorithm to run"
    )

    args = parser.parse_args()

    data1 = [100,5,10,1,11]
    data2 = [1,2,-3,4,5]
    data3 = ["k",2,1]


    user_input = input(
        "Enter integers separated by spaces or commas: "
    )

    # Convert input into a list (leave validation to the algorithm)
    raw_values = user_input.replace(",", " ").split()
    arr = []

    try:
        arr = [int(x) for x in user_input.replace(",", " ").split()]
    except ValueError:
        print("Invalid input")
        exit(1)
    
    if args.algorithm == "QUICKSORT":
        result1 = quick_sort(data1.copy())
        result2 = quick_sort(data2.copy())
        result3 = quick_sort(data3.copy())
        result4 = quick_sort(arr.copy())

    
    print("Test Array 1:")
    print("Original:", data1)
    print("Sorted:  ", result1)
    
    print("")
    print("Test Array 2:")
    print("Original:", data2)
    print("Sorted:  ", result2)
    
    print("")
    print("Test Array 3:")
    print("Original:", data3)
    print("Sorted:  ", result3)

    print("")
    print("User Test Array:")
    print("Original:", arr)
    print("Sorted:  ", result4)

if __name__ == "__main__":
    main()
