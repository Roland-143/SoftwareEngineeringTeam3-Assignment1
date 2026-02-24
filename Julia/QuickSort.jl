function quick_sort(arr)
    if length(arr) <= 1
        return arr
    end

    pivot = arr[1]
    less = [x for x in arr[2:end] if x <= pivot]
    greater = [x for x in arr[2:end] if x > pivot]

    return vcat(quick_sort(less), [pivot], quick_sort(greater))
end
println("Enter numbers separated by spaces:")
input = readline()
arr = parse.(Int, split(input))

sorted = quick_sort(arr)

println("Sorted array:")
println(sorted)
