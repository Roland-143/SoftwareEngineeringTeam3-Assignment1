
def insertion_sort(arr):
    # Validate that the input is valid
    for item in arr:
        if not isinstance(item, int):
            return None

    # Traverse from the second element to the end
    for i in range(1, len(arr)):
        key = arr[i]          # Element to be inserted
        j = i - 1

        # Move elements greater than the key one position ahead
        while j >= 0 and arr[j] > key:
            arr[j + 1] = arr[j]
            j -= 1

        # Insert the key at its correct position
        arr[j + 1] = key

    return arr


