fn main() {
	let args: Vec<_> = std::env::args().collect();
	let n: i32 = if args.len() == 2 {
			args[1].parse::<i32>().unwrap_or(0)
		} else {
			1
		};
	println!("Ack(3, {}): {}", n, ackermann(3, n));
}

fn ackermann(m: i32, n: i32) -> i32 {
	if m == 0 {
		n + 1
	} else if n == 0 {
		ackermann(m - 1, 1)
	} else {
		ackermann(m - 1, ackermann(m, n - 1))
	}
}