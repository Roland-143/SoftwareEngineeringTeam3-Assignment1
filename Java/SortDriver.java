import java.util.Arrays;

/**
 * SortDriver.java
 *
 * Driver program that runs a selected sorting algorithm from the command line.
 *
 * The user selects which algorithm to run via a command-line argument.
 * The driver runs the chosen algorithm on a few datasets.
 * Each algorithm method returns a sorted array (or null if input is invalid).
 *
 * Usage:
 *   javac *.java
 *   java SortDriver BUBBLESORT
 *   java SortDriver MERGESORT
 *
 * Algorithms:
 *   MERGESORT, QUICKSORT, INSERTIONSORT, BUBBLESORT, SELECTIONSORT
 */
public class SortDriver {

    /**
     * Enum to constrain valid algorithm names and make command-line parsing simple.
     */
    private enum Algorithm {
        MERGESORT, QUICKSORT, INSERTIONSORT, BUBBLESORT, SELECTIONSORT
    }

    public static void main(String[] args) {

        // Must supply exactly one argument: the algorithm name
        if (args.length != 1) {
            printUsage();
            return;
        }

        // Convert user input into one of the allowed enum values
        Algorithm algo;
        try {
            algo = Algorithm.valueOf(args[0].toUpperCase());
        } catch (IllegalArgumentException ex) {
            System.out.println("Invalid algorithm: " + args[0]);
            printUsage();
            return;
        }

        // Sample datasets
        int[] data1 = {100, 5, 10, 1, 11};
        int[] data2 = {1, 2, -3, 4, 5};

        int[] result1 = runSort(algo, data1);
        int[] result2 = runSort(algo, data2);

        // Print results
        printCase("Original:", data1, result1);
        System.out.println();
        printCase("Original:", data2, result2);
        System.out.println();
    }

    /**
     * Runs the selected sorting algorithm.
     *
     * Our algorithm classes use the Python-style API:
     *   int[] sorted = AlgorithmName.sort(int[] arr)
     *
     * - Returns a sorted array (new array) if successful
     * - Returns null if input is invalid (e.g., null input)
     *
     * @param algo     the algorithm to run
     * @param original input array (may be null)
     * @return sorted array result, or null if invalid
     */
    private static int[] runSort(Algorithm algo, int[] original) {
        if (original == null) {
            return null;
        }

        // Each sort(...) returns a sorted array (does not modify the original)
        return switch (algo) {
            case SELECTIONSORT -> SelectionSort_ID30Zachary.sort(original);
            case INSERTIONSORT -> InsertionSort_ID30Zachary.sort(original);
            case BUBBLESORT -> BubbleSort_ID30Zachary.sort(original);
            case MERGESORT -> MergeSort_ID30Zachary.sort(original);
            case QUICKSORT -> QuickSort_ID30Zachary.sort(original);
        };
    }

    /**
     * Prints one dataset before/after.
     */
    private static void printCase(String label, int[] original, int[] result) {
        System.out.println(label + " " + Arrays.toString(original));
        if (result != null) {
            System.out.println("Sorted:   " + Arrays.toString(result));
        } else {
            System.out.println("Invalid input");
        }
    }

    /**
     * Attempts to parse a String[] into an int[].
     * Returns null if any element is not a valid integer.
     */
    private static int[] parseIntArrayOrNull(String[] raw) {
        if (raw == null) {
            return null;
        }

        int[] out = new int[raw.length];

        for (int i = 0; i < raw.length; i++) {
            try {
                out[i] = Integer.parseInt(raw[i]);
            } catch (NumberFormatException ex) {
                // Any non-integer token makes the whole dataset invalid
                return null;
            }
        }

        return out;
    }

    /**
     * Prints command-line usage help.
     */
    private static void printUsage() {
        System.out.println("Usage: java SortDriver <ALGORITHM>");
        System.out.println("Where <ALGORITHM> is one of:");
        System.out.println("  MERGESORT, QUICKSORT, INSERTIONSORT, BUBBLESORT, SELECTIONSORT");
        System.out.println("Example:");
        System.out.println("  java SortDriver MERGESORT");
    }
}
