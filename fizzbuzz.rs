#!/usr/local/bin/rust

fn is_three(num: int) -> bool {
	num % 3 == 0
}

fn is_five(num: int) -> bool {
	num % 5 == 0
}

fn is_fifteen(num: int) -> bool {
	num % 15 == 0
}

fn for_loop() {
	for num in range(1, 101) {
		println(
			if (num % 15 == 0) { ~"FizzBuzz" }
			else if (num % 3 == 0) { ~"Fizz" }
			else if (num % 5 == 0) { ~"Buzz" }
			else { num.to_str() }
		);
	}
}

fn while_loop() {
	let mut num:uint = 1;
	while num <= 100 {
		println(
			if (num % 15 == 0) { ~"FizzBuzz" }
			else if (num % 3 == 0) { ~"Fizz" }
			else if (num % 5 == 0) { ~"Buzz" }
			else { num.to_str() }
		);
		num += 1;
	}
}

fn first() {
	for num in range(1, 101) {
		let mut answer = "";

		if is_fifteen(num) {
			answer = "FizzBuzz";
		} else if is_three(num) {
			answer = "Fizz";
		} else if is_five(num) {
			answer = "Buzz";
		} else {
			answer = "";
		}

		if (answer != "") {
			println(answer);
		} else {
			println(num.to_str());
		}
	}
}

fn second() {
	for num in range(1, 101) {
		let answer = 
		if is_fifteen(num) {
			"FizzBuzz"
		} else if is_three(num) {
			"Fizz"
		} else if is_five(num) {
			"Buzz"
		} else {
			""
		};

		if (answer != "") {
			println(answer);
		} else {
			println(num.to_str());
		}
		
	}
}

fn third() {
	for num in range(1, 101) {
		println(
			if is_fifteen(num) { ~"FizzBuzz" }
			else if is_three(num) { ~"Fizz" }
			else if is_five(num) { ~"Buzz" }
			else { num.to_str() }
		);
	}
}

fn main() {
	//for_loop();
	//while_loop();
	//first();
	//second();
	third();
}
