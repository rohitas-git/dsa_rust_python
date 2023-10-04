
#! Stability

# It means that 
#   if two items have same value, 
#   then they should appear in the same order as they appeared in the original array

# A sorting algorithm is said to be stable 
# if two objects with equal keys appear in the same order in sorted output 
# as they appear in the input data set.

arr = [("ab",50), ("rb",90),("ad",50),("zb",90)]

# sorting according to marks
arrSortedStable = [("ab",50), ("ad",50), ("rb",90), ("zb",90)] 
arrSortedUnstable = [("ad",50), ("ab",50), ("zb",90), ("rb",90)] 

#! Stability is important when you have multi-field objects 
# and you are sorting according a set of fields, one or two fields
# and want other fields to follow the original order

# Stability not important when sorting array of integers

# Stable Sorts: Bubble Sort, Insertion Sort, Merge Sort, Count sort...

# Unstable Sorts: Selection Sort, Heap Sort, Quick Sort, ...

# Quick Sort, Heap Sort etc., can be made stable by also taking the position of the elements into consideration