#!/bin/bash

for file in src/examples/*
do
	echo "testing file $file"
	./pseudotex -i $file -o test.tex
done

rm test.*
