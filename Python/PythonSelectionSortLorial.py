def selection_sort(arr):
    # Validate that the input is valid
    for item in arr:
        if not isinstance(item, int):
            return None
    n = len(arr)

    # Move the boundary of the sorted portion
    for i in range(n):
        min_index = i

        # Find the smallest element in the unsorted portion
        for j in range(i + 1, n):
            if arr[j] < arr[min_index]:
                min_index = j

        # Swap the found smallest element with the first unsorted element
        arr[i], arr[min_index] = arr[min_index], arr[i]

    return arr
