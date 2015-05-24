const IM: i64 = 139968;
const IA: i64 = 3877;
const IC: i64 = 29573;
static mut last: i64 = 42;

fn main() {
	let args: Vec<_> = std::env::args().collect();
	let n: usize = if args.len() == 2 {
			args[1].parse::<usize>().unwrap_or(0)
		} else {
			1
		};
	let mut ary: Vec<f64> = Vec::with_capacity(n + 1);
	ary.push(0.0);
	unsafe {
		for _ in 1..(n + 1) {
			ary.push(gen_random(1.0));
		}
	}
	heapsort(n as i32, &mut ary);
	println!("{:.*}", 10, ary[n]);
}

fn heapsort(n: i32, ra: &mut Vec<f64>) {
	let mut ir = n;
	let mut l = (n >> 1) + 1;
	let mut rra;
	let mut i;
	let mut j;

	loop {
		if l > 1 {
			l -= 1;
			rra = ra[l as usize];
		} else {
			rra = ra[ir as usize];
			ra[ir as usize] = ra[1];
			ir -= 1;
			if ir == 1 {
				ra[1] = rra;
				return;
			}
		}
		i = l;
		j = l << 1;
		while j <= ir {
			if j < ir && ra[j as usize] < ra[(j + 1) as usize] {
				j += 1;
			}
			if rra < ra[j as usize] {
				ra[i as usize] = ra[j as usize];
				i = j;
				j += i;
			} else {
				j = ir + 1;
			}
		}
		ra[i as usize] = rra;
	}
}

unsafe fn gen_random(max: f64) -> f64 {
	last = (last * IA + IC) % IM;
	max * (last as f64) / (IM as f64)
}