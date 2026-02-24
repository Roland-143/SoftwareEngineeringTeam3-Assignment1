import java.util.Arrays;

/**
 * InsertionSort_ID30Zachary.java
 *
 * Implements the Insertion Sort algorithm.
 * Returns a new sorted array.
 */
public class InsertionSort_ID30Zachary {

    /**
     * Sorts the input array using Insertion Sort.
     *
     * @param arr the input array
     * @return a new sorted array, or null if input is null
     */
    public static int[] sort(int[] arr) {

        if (arr == null) {
            return null;
        }

        int[] copy = Arrays.copyOf(arr, arr.length);

        // Iterate through array starting from second element
        for (int i = 1; i < copy.length; i++) {

            int key = copy[i];     // Current element to insert
            int j = i - 1;

            // Shift elements greater than key one position to the right
            while (j >= 0 && copy[j] > key) {
                copy[j + 1] = copy[j];
                j--;
            }

            // Insert key into its correct location
            copy[j + 1] = key;
        }

        return copy;
    }
}
