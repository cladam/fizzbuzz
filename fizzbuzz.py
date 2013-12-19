#!/usr/bin/env python

for num in range(1,101):
   answer = ""
   if not (num%15):
      answer = "FizzBuzz"
   elif not (num%3):
      answer = "Buzz"
   elif not (num%5):
      answer = "Fizz"
   print "Number: " + str(num) + " is: " + answer
