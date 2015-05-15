#!/bin/sh

for file in ./*.rs
do
	out=`echo $file | sed s/.rs//`

	echo "Compile for $file with -O3"
	rustc $file -C opt-level=3 -o ../target/${out}
done

