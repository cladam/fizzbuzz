#!/usr/bin/env bash

for num in {1..100}
  do 
	([ $((num%15)) -eq 0 ] && echo "FizzBuzz") || 
	([ $((num%3)) -eq 0 ] && echo "Fizz") || 
	([ $((num%5)) -eq 0 ] && echo "Buzz") ||
	echo $num
done