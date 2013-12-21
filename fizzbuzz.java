class fizzbuzz {
	public static void main(String... args) {
		String answer;
		for (int i = 1; i < 101; ++i) {
			if (i%15 == 0) {
				answer = "FizzBuzz";
			} else if (i%3 == 0) {
				answer = "Fizz";
			} else if (i%5 == 0) {
				answer = "Buzz";
			} else {
				answer = Integer.toString(i);
			}
			System.out.println(answer);
		}
	}
}
