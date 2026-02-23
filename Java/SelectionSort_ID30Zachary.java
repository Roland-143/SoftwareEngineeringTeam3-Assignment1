import java.util.Arrays;

/**
 * SelectionSort_ID30Zachary.java
 *
 * Implements the Selection Sort algorithm.
 * Returns a new sorted array.
 */
public class SelectionSort_ID30Zachary {

    /**
     * Sorts the input array using Selection Sort.
     *
     * @param arr the input array
     * @return a new sorted array, or null if input is null
     */
    public static int[] sort(int[] arr) {

        if (arr == null) {
            return null;
        }

        int[] copy = Arrays.copyOf(arr, arr.length);

        // Move boundary of unsorted portion
        for (int i = 0; i < copy.length - 1; i++) {

            int minIndex = i;

            // Find smallest element in remaining unsorted portion
            for (int j = i + 1; j < copy.length; j++) {
                if (copy[j] < copy[minIndex]) {
                    minIndex = j;
                }
            }

            // Swap smallest element with current position
            int temp = copy[i];
            copy[i] = copy[minIndex];
            copy[minIndex] = temp;
        }

        return copy;
    }
}
