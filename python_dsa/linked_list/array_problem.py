
# ! Problems with Array
# - Either size is fixed and preallocated (in both fixed and variable sized array)
# OR the worst case insertion at the end is Theta(N) 
# - Insertion at middle (or beginning) is costly 
# - Deletion at middle (or beginning) is costly 
# - Implementation of data structures like queue and deque is complex with arrays

# 1. How to implement Round Robin Scheduling?
# - have to select an item and remove it from queue
# - have to remove it from its curr position and move it to the end

# 2. Given a sequence of items. Whenever we see an item x in the sequence,
# we need to replace it with 5 instances of another item y 
# - Multiple insertion in middle which are problematic with array
# - Need to allocate a new instance of array for it, 
# difficult to do it in same implementation

# 3. We have multiple sorted sequences and we need merge them frequently 
# - If mergesort with array, need aux space 
# - If with linkedlist, no aux space needed in the same time

# 4. In system level programming, when needed to work with fragmented memory

# -----------------------------------------------------------

# Details of Round Robin Schedulingw: 
# Program for Round Robin scheduling

# Round Robin is a CPU scheduling algorithm where each process is assigned a fixed time slot in a cyclic way. 
# It is basically the preemptive version of First come First Serve CPU Scheduling algorithm. 

# Round Robin CPU Algorithm generally focuses on Time Sharing technique. 
# The period of time for which a process or job is allowed to run in a pre-emptive method is called time quantum. 
# Each process or job present in the ready queue is assigned the CPU for that time quantum, 
# if the execution of the process is completed during that time 
# then the process will end else the process will go back to the waiting table 
#   and wait for its next turn to complete the execution.