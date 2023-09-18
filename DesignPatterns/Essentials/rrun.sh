#!/bin/bash

filePath=$1
outputFilename=$2

rustc ./$1.rs -o $PWD/bin/$2  && $PWD/bin/$2