import java.util.Arrays;

/**
 * QuickSort_ID30Zachary.java
 *
 * Implements the Quick Sort algorithm.
 * Returns a new sorted array.
 */
public class QuickSort_ID30Zachary {

    /**
     * Sorts the input array using Quick Sort.
     *
     * @param arr the input array
     * @return a new sorted array, or null if input is null
     */
    public static int[] sort(int[] arr) {

        if (arr == null) {
            return null;
        }

        int[] copy = Arrays.copyOf(arr, arr.length);

        quickSort(copy, 0, copy.length - 1);

        return copy;
    }

    // Recursively sorts elements using partitioning.
    private static void quickSort(int[] arr, int low, int high) {

        if (low < high) {

            int pivotIndex = partition(arr, low, high);

            // Sort elements before pivot
            quickSort(arr, low, pivotIndex - 1);

            // Sort elements after pivot
            quickSort(arr, pivotIndex + 1, high);
        }
    }

    /**
     * Partitions array around a pivot element.
     * Elements smaller than pivot go left,
     * larger elements go right.
     */
    private static int partition(int[] arr, int low, int high) {

        int pivot = arr[high];  // Choose last element as pivot
        int i = low - 1;

        for (int j = low; j < high; j++) {

            if (arr[j] < pivot) {
                i++;

                int temp = arr[i];
                arr[i] = arr[j];
                arr[j] = temp;
            }
        }

        // Place pivot in correct sorted position
        int temp = arr[i + 1];
        arr[i + 1] = arr[high];
        arr[high] = temp;

        return i + 1;
    }
}
