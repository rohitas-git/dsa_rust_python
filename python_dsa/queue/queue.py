
# Queue Data structure
# A queue is a linear data structure that is open at both ends 
# and the operations are performed in First In First Out (FIFO) order.

# Operations in Queue
# - enqueue, dequeue, getFront, getRear, size, isEmpty 

# Characteristics of Queue:
# - Queue can handle multiple data.
# - We can access both ends.
# - They are fast and flexible. 

# When to use queue data structure
# 1. When there is a single resource and multiple consumers 
#       and want consumers to be served in FIFO manner
# 2. When need to synchronize a slow and fast device
# When data is transferred asynchronously (data not necessarily received at the same rate as sent) between two processes. 

# Queue in Python
# - using list
# - using collections.deque 
# - using queue.Queue (queue is thread-safe implementations to use in multi-thread env)   
# - our own implementation

# 1. using list for Queue
#   Dequeue - O(n), Enqueue - O(1) => Inefficient impl of queue

# 2. using deque (impl using doubly linked list)
#   Dequeue - Theta(1), Enqueue - O(1) => Efficient impl of queue (recommended in python)

