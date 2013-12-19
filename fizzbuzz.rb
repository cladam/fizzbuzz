#!/usr/bin/env ruby
(1..100).each do |n|
  if n % 3 == 0
    puts "Fizz"
  elsif n % 5 == 0
    puts "Buzz"
  elsif n % 15 == 0
    puts "FizzBuzz"
  else
    puts n
  end
end