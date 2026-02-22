def bubble_sort(arr):
    # Validate that the input is valid
    for item in arr:
        if not isinstance(item, int):
            return None
    n = len(arr)

    # Traverse through all array elements
    for i in range(n):
        swapped = False

        # Last i elements are already in place
        for j in range(0, n - i - 1):
            if arr[j] > arr[j + 1]:
                # Swap
                arr[j], arr[j + 1] = arr[j + 1], arr[j]
                swapped = True

        # If the array is already sorted stop
        if not swapped:
            break

    return arr


