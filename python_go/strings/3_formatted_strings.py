
# * Formatted strings
# WHY - use when need to print formatted data
# HOW 
# - using % (C lang style)
# - using format() fn
# - using f-string [BEST WAY]

name ="raj"
course = "python"

s1 = "welcome %s to %s"%(course,name)
s2 = "welcome {1} to {0}".format(course,name)
s3 = f"welcome {name.lower()} to {course.upper()}"

