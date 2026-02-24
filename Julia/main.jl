include("BubbleSort.jl")
include("SelectionSort.jl")
include("InsertionSort.jl")
include("MergeSort.jl")
include("QuickSort.jl")

function main()
    if length(ARGS) == 0
        println("Usage: julia main.jl bubble|selection|insertion|merge|quick")
        return
    end

    algo = lowercase(ARGS[1])
    arr = [64, 34, 25, 12, 22, 11, 90]

    sorted =
        if algo == "bubble"
            bubble_sort(arr)
        elseif algo == "selection"
            selection_sort(arr)
        elseif algo == "insertion"
            insertion_sort(arr)
        elseif algo == "merge"
            merge_sort(arr)
        elseif algo == "quick"
            quick_sort(arr)
        else
            println("Unknown algorithm: $algo")
            return
        end

    println("Algorithm: $algo")
    println("Original: ", arr)
    println("Sorted:   ", sorted)
end

main()