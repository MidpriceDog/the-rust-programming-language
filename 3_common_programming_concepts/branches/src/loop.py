import timeit

# Define the code snippet you want to measure

code_to_measure = """
arr = 1_000_000 * [4]
result = 0
i = 0
while i < len(arr):
    result += arr[i]
    i += 1
"""

# Measure the execution time
execution_time = timeit.timeit(code_to_measure, number=1)

print("Execution time:", execution_time * 1_000_000, "µs")
