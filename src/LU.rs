const IM: i64 = 139968;
const IA: i64 = 3877;
const IC: i64 = 29573;
static mut last: i64 = 42;

const MAT_SIZE: usize = 1500;

fn main() {
	let mut a: Vec<Vec<f32>> = Vec::with_capacity(MAT_SIZE);
	let mut l: Vec<Vec<f32>> = Vec::with_capacity(MAT_SIZE);
	let mut u: Vec<Vec<f32>> = Vec::with_capacity(MAT_SIZE);

	a.extend(std::iter::repeat(Vec::with_capacity(MAT_SIZE)).take(MAT_SIZE));
	l.extend(std::iter::repeat(Vec::with_capacity(MAT_SIZE)).take(MAT_SIZE));
	u.extend(std::iter::repeat(Vec::with_capacity(MAT_SIZE)).take(MAT_SIZE));

	unsafe {
		for i in 0..MAT_SIZE {
			for _ in 0..MAT_SIZE {
				a[i].push(gen_random(1.0));
			}
		}
	}
	lu(&a, &mut l, &mut u);
	println!("{}, {}", l[25][25], u[25][25]);
}

fn lu(a: & Vec<Vec<f32>>, l: &mut Vec<Vec<f32>>, u: &mut Vec<Vec<f32>>) {
	for i in 0..MAT_SIZE {
		for j in 0..MAT_SIZE {
			if j < i {
				l[j].push(0.0);
			} else {
				l[j].push(a[j][i]);
				for k in 0..i {
					l[j][i] = l[j][i] - l[j][k] * u[k][i];
				}
			}
		}
		for j in 0..MAT_SIZE {
			if j < i {
				u[i].push(0.0);
			} else if j == i {
				u[i].push(1.0);
			} else {
				u[i].push(a[i][j] / l[i][i]);
				for k in 0..i {
					u[i][j] = u[i][j] - ((l[i][k] * u[k][j]) / l[i][i]);
				}
			}
		}
	}
}

unsafe fn gen_random(max: f32) -> f32 {
	last = (last * IA + IC) % IM;
	max * (last as f32) / (IM as f32)
}