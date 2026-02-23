import java.util.Arrays;

/**
 * MergeSort_ID30Zachary.java
 *
 * Implements the Merge Sort algorithm.
 * Returns a new sorted array.
 */
public class MergeSort_ID30Zachary {

    /**
     * Sorts the input array using Merge Sort.
     *
     * @param arr the input array
     * @return a new sorted array, or null if input is null
     */
    public static int[] sort(int[] arr) {

        if (arr == null) {
            return null;
        }

        int[] copy = Arrays.copyOf(arr, arr.length);

        mergeSort(copy, 0, copy.length - 1);

        return copy;
    }

    // Recursively divides the array into halves.
    private static void mergeSort(int[] arr, int left, int right) {

        if (left < right) {

            int mid = (left + right) / 2;

            // Sort left half
            mergeSort(arr, left, mid);

            // Sort right half
            mergeSort(arr, mid + 1, right);

            // Merge the two sorted halves
            merge(arr, left, mid, right);
        }
    }

    /**
     * Merges two sorted subarrays into one sorted section.
     */
    private static void merge(int[] arr, int left, int mid, int right) {

        int n1 = mid - left + 1;
        int n2 = right - mid;

        int[] L = new int[n1];
        int[] R = new int[n2];

        // Copy data into temporary arrays
        for (int i = 0; i < n1; i++)
            L[i] = arr[left + i];

        for (int j = 0; j < n2; j++)
            R[j] = arr[mid + 1 + j];

        int i = 0, j = 0, k = left;

        // Merge temp arrays back into original
        while (i < n1 && j < n2) {
            if (L[i] <= R[j])
                arr[k++] = L[i++];
            else
                arr[k++] = R[j++];
        }

        // Copy remaining elements if any
        while (i < n1)
            arr[k++] = L[i++];

        while (j < n2)
            arr[k++] = R[j++];
    }
}
