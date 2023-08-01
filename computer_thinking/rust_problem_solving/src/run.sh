#!/bin/bash

file=$1
rustc "$file.rs" && "./$file"
rm -f "./$file"