package main

func BubbleSort(a []int) {
	n := len(a)
	if n < 2 {
		return
	}

	for i := 0; i < n-1; i++ {
		swapped := false
		for j := 0; j < n-1-i; j++ {
			if a[j] > a[j+1] {
				a[j], a[j+1] = a[j+1], a[j]
				swapped = true
			}
		}
		if !swapped {
			return
		}
	}
}