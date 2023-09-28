
s1 = {2,4,6,8}
s2 = {4,8}

# disjoint - true if no common value btw two sets
print(s1.isdisjoint(s2))

# subset operation (<=): s1.issubset(s2)
print(s1 <= s2)

# proper subset operation (<)
print(s1 < s2)

# superset (>=) / s1.issuperset(s2)
print(s1 >= s2)

# proper superset (>)
print(s1 > s2)
