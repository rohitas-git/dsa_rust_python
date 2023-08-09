#!/bin/bash

# To run a single rust file 

file=$1
rustc "$file.rs" && "./$file"
echo "Running $file"
rm -f "./$file"

