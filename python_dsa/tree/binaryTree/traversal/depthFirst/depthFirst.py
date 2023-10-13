
# * Depth First  
    
# * Process the whole tree, 3 THINGS to do:
#  * Process root node
#  * Process left subtree as we did the whole tree 
#  * Process right subtree as we did the whole tree 

# Number of ways to process whole tree == no. of ways to do 3 things => 3!
# Most popular ways are 3 WAYS:
    # - Inorder
    # - Preorder
    # - Postorder
    
# In all 3 popular ways of traversal, 
# we visit left subtree before the right subtree 

# Root | Left | Right    => Preorder
# Left | Root | Right    => Inorder
# Left | Right | Root    => Postorder

# Note: When recursively call for a subtree, 
#   if subtree has single node, you print it
#   otherwise you follow the same process recursively

 
 