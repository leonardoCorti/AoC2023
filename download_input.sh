#!/bin/bash

file_path="inputs/$1.txt"
url="https://adventofcode.com/2023/day/$1/input"
cookie="Cookie: session=$2"

if [ -e "$file_path" ]; then
	echo "File already exists"
else
	echo "downloading day $1"
	curl -o "$file_path" -H "$cookie" "$url"
	echo "done downloading"
fi
