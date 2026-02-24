package main

func MergeSort(a []int) []int {
	if len(a) <= 1 {
		return cloneLocal(a)
	}

	mid := len(a) / 2
	left := MergeSort(a[:mid])
	right := MergeSort(a[mid:])

	return merge(left, right)
}

func merge(left, right []int) []int {
	out := make([]int, 0, len(left)+len(right))
	i, j := 0, 0

	for i < len(left) && j < len(right) {
		if left[i] <= right[j] {
			out = append(out, left[i])
			i++
		} else {
			out = append(out, right[j])
			j++
		}
	}
	out = append(out, left[i:]...)
	out = append(out, right[j:]...)
	return out
}

func cloneLocal(a []int) []int {
	out := make([]int, len(a))
	copy(out, a)
	return out
}