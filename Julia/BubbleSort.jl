function bubble_sort(arr)
    n = length(arr)
    a = copy(arr)

    for i in 1:n-1
        for j in 1:n-i
            if a[j] > a[j+1]
                a[j], a[j+1] = a[j+1], a[j]
            end
        end
    end

    return a
end

println("Enter numbers separated by spaces:")
input = readline()
arr = parse.(Int, split(input))

sorted = bubble_sort(arr)

println("Sorted array:")
println(sorted)
