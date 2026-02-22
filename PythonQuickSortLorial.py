def quick_sort(arr):
    # Validate that the input is valid
    for item in arr:
        if not isinstance(item, int):
            return None

    # Base case
    if len(arr) <= 1:
        return arr

    # Choose a pivot (middle element)
    pivot = arr[len(arr) // 2]

    # Partition the array
    left = [x for x in arr if x < pivot]
    middle = [x for x in arr if x == pivot]
    right = [x for x in arr if x > pivot]

    # Recursively sort and combine
    return quick_sort(left) + middle + quick_sort(right)
