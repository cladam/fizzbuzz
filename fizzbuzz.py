#!/usr/bin/env python

for num in range(1,101):
	if (num % 15 == 0):
		answer = "FizzBuzz"
	elif (num % 3 == 0):
		answer = "Fizz"
	elif (num % 5 == 0):
		answer = "Buzz"
	else:
		answer = num
	print answer
