def merge_sort(arr):
    # Validate that the input is valid
    for item in arr:
        if not isinstance(item, int):
            return None

    # Base case: The array is of size 0 or 1 so it is already sorted
    if len(arr) <= 1:
        return arr

    # Divide the array into two halves
    mid = len(arr) // 2
    left = arr[:mid]
    right = arr[mid:]

    # Merge the sorted halves of the array
    return merge(merge_sort(left), merge_sort(right))


def merge(left, right):
    result = []
    i = j = 0

    # Merge the elements from left and right in sorted order
    while i < len(left) and j < len(right):
        if left[i] <= right[j]:
            result.append(left[i])
            i += 1
        else:
            result.append(right[j])
            j += 1

    result.extend(left[i:])
    result.extend(right[j:])
    return result



