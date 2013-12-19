#!/usr/bin/env python

for num in range(1,101):
   answer = ""
   if (num % 15 == 0):
      answer = "FizzBuzz"
   elif (num % 3 == 0):
      answer = "Fizz"
   elif (num % 5 == 0):
      answer = "Buzz"
   print "Number: " + str(num) + " is: " + answer
