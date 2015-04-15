fn main() {
	let args: Vec<_> = std::env::args().collect();
	let n: u64 = if args.len() == 2 {
			args[1].parse::<u64>().unwrap_or(0)
		} else {
			1
		};
	println!("{}", fib(n));
}

fn fib(n: u64) -> u64 {
	if n < 2 {
		1
	} else {
		fib(n - 2) + fib(n - 1)
	}
}