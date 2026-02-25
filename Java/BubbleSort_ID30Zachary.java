import java.util.Arrays;

/**
 * BubbleSort_ID30Zachary.java
 *
 * Implements the Bubble Sort algorithm.
 * Returns a new sorted array.
 */
public class BubbleSort_ID30Zachary {

    /**
     * Sorts the input array using Bubble Sort.
     *
     * @param arr the input array
     * @return a new sorted array, or null if input is null
     */
    public static int[] sort(int[] arr) {

        // Validate input
        if (arr == null) {
            return null;
        }

        // Work on a copy so original array is not modified
        int[] copy = Arrays.copyOf(arr, arr.length);

        int n = copy.length;
        boolean swapped;

        // Perform passes through the array
        for (int i = 0; i < n - 1; i++) {

            swapped = false;

            // Compare adjacent elements
            for (int j = 0; j < n - i - 1; j++) {

                if (copy[j] > copy[j + 1]) {
                    int temp = copy[j];
                    copy[j] = copy[j + 1];
                    copy[j + 1] = temp;
                    swapped = true;
                }
            }

            // Optimization: stop early if no swaps occurred
            if (!swapped) {
                break;
            }
        }

        return copy;
    }
}