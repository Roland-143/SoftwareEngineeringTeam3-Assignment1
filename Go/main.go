package main

import (
	"fmt"
	"os"
	"strings"
)

func main() {
	if len(os.Args) < 2 {
		printUsageAndExit()
	}
	alg := strings.ToUpper(strings.TrimSpace(os.Args[1]))

	// Test cases live in main (as requested)
	testCases := [][]int{
		{5, 3, 8, 4, 2},
		{1, 2, 3, 4, 5},
		{5, 4, 3, 2, 1},
		{10, -1, 3, 3, 7, 0, -5},
		{},
		{42},
		{9, 9, 9, 1, 9},
	}

	fmt.Println("Algorithm:", alg)
	fmt.Println("====================================")

	for i, tc := range testCases {
		original := cloneSlice(tc)
		arr := cloneSlice(tc)

		switch alg {
		case "BUBBLE":
			BubbleSort(arr)
		case "INSERTION":
			InsertionSort(arr)
		case "SELECTION":
			SelectionSort(arr)
		case "MERGE":
			arr = MergeSort(arr) // returns new slice
		case "QUICK":
			QuickSort(arr) // in-place
		default:
			fmt.Printf("Unknown algorithm: %s\n\n", alg)
			printUsageAndExit()
		}

		fmt.Printf("Test #%d\n", i+1)
		fmt.Printf("  Before: %v\n", original)
		fmt.Printf("  After : %v\n", arr)
		fmt.Printf("  Sorted: %v\n", isSorted(arr))
		fmt.Println("------------------------------------")
	}
}

func printUsageAndExit() {
	fmt.Println("Usage:")
	fmt.Println("  go run . <ALGORITHM>")
	fmt.Println()
	fmt.Println("ALGORITHM options:")
	fmt.Println("  BUBBLE | INSERTION | SELECTION | MERGE | QUICK")
	os.Exit(1)
}

func cloneSlice(a []int) []int {
	out := make([]int, len(a))
	copy(out, a)
	return out
}

func isSorted(a []int) bool {
	for i := 1; i < len(a); i++ {
		if a[i-1] > a[i] {
			return false
		}
	}
	return true
}