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
		println!("Number: {} is = {}", num,
			if (num % 15 == 0) { ~"FizzBuzz" }
			else if (num % 3 == 0) { ~"Buzz" }
			else if (num % 5 == 0) { ~"Fizz" }
			else { ~"" }
		);
	}
}

fn while_loop() {
	let mut num:uint = 1;
	while num <= 100 {
		println!("Number: {} is = {}", num,
			if (num % 15 == 0) { ~"FizzBuzz" }
			else if (num % 3 == 0) { ~"Buzz" }
			else if (num % 5 == 0) { ~"Fizz" }
			else { ~"" }
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
			answer = "Buzz";
		} else if is_five(num) {
			answer = "Fizz";
		} else {
			answer = "";
		}
		println!("Number: {} is: {}", num, answer);
	}
}

fn second() {
	for num in range(1, 101) {
		let answer = 
		if is_fifteen(num) {
			"FizzBuzz"
		} else if is_three(num) {
			"Buzz"
		} else if is_five(num) {
			"Fizz"
		} else {
			""
		};
		if (answer != "") {
			println!("Number: {} is: {}", num, answer);
		}
	}
}

fn third() {
	for num in range(1, 101) {
		println!("Number: {} is = {}", num,
			if is_fifteen(num) { ~"FizzBuzz" }
			else if is_three(num) { ~"Buzz" }
			else if is_fifteen(num) { ~"Fizz" }
			else { ~"" }
		);
	}
}

fn main() {
	//for_loop();
		/*
		real	0m0.006s
		user	0m0.002s
		sys		0m0.005s
		*/
	//while_loop();
		/*
		real	0m0.006s
		user	0m0.002s
		sys		0m0.005s
		*/
	//first();
		/*
		real	0m0.006s
		user	0m0.002s
		sys		0m0.005s
		*/
	//second();
		/*
		real	0m0.006s
		user	0m0.002s
		sys		0m0.005s
		*/
	//third();
		/*
		real	0m0.006s
		user	0m0.002s
		sys		0m0.005s
		*/
}
