use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::VecDeque;
use std::collections::BTreeSet;

const NODES_COUNT: usize = 5000;

type VertexTyp = i32;
type WeightTyp = f64;

const MAX_WEIGHT: WeightTyp = std::f64::INFINITY;

#[derive(Clone)]
struct Neighbor {
    target: VertexTyp,
    weight: WeightTyp
}

impl Neighbor {
	fn new(target: VertexTyp, weight: WeightTyp) -> Neighbor {
		Neighbor { target: target, weight: weight }
	}
}

// http://stackoverflow.com/questions/28247990/how-to-do-a-binary-search-on-a-vec-of-floats
#[derive(PartialEq, PartialOrd, Clone)]
struct NonNan(f64);
impl Eq for NonNan {}
impl Ord for NonNan {
	fn cmp(&self, other: &NonNan) -> std::cmp::Ordering {
		self.partial_cmp(other).unwrap()
	}
}

type AdjListTyp = Vec<Vec<Neighbor>>;

fn main() {
	let file = File::open("others/test.csv").unwrap();
	let reader = BufReader::new(&file);

	let mut adj_list: AdjListTyp = Vec::with_capacity(NODES_COUNT);
	adj_list.extend(std::iter::repeat(Vec::new()).take(NODES_COUNT));

	for line in reader.lines() {
		let res = line.unwrap();
		let mut split = res.split(",");
		let from = split.next().unwrap().parse::<usize>().unwrap();
		let to = split.next().unwrap().parse::<VertexTyp>().unwrap();
		let weight = split.next().unwrap().parse::<WeightTyp>().unwrap();
		
		adj_list[from].push(Neighbor::new(to, weight));
	}

	let mut min_dist: Vec<WeightTyp> = Vec::new();
	let mut previous: Vec<VertexTyp> = Vec::new();
	dijkstra_compute_paths(0, &adj_list, &mut min_dist, &mut previous);
	println!("Distance from 0 to 4: {}", min_dist[4]);

	let path: VecDeque<VertexTyp> = dijkstra_shortest_path(4, &previous);
	print!("Path : ");
	for node in path {
		print!("{} ", node);
	}
	println!("");
}

fn dijkstra_compute_paths(source: VertexTyp, adj_list: &AdjListTyp,
							min_dist: &mut Vec<WeightTyp>, previous: &mut Vec<VertexTyp>) {
	min_dist.clear();
	min_dist.extend(std::iter::repeat(MAX_WEIGHT).take(NODES_COUNT));

	min_dist[source as usize] = 0.0;
	
	previous.clear();
	previous.extend(std::iter::repeat(-1).take(NODES_COUNT));

	let mut vertex_queue: BTreeSet<(NonNan, VertexTyp)> = BTreeSet::new();
	vertex_queue.insert((NonNan(min_dist[source as usize]), source));

	while !vertex_queue.is_empty() {
		let first = vertex_queue.iter().cloned().next().unwrap();
		vertex_queue.remove(&first);
		let (NonNan(dist), u) = first;

		// Visit each edge exiting first
		let neighbors = &adj_list[first.1 as usize];
		for neighbor in neighbors {
			let v = neighbor.target as usize;
			let weight = neighbor.weight;

			let dist_through_first = dist + weight;
			if dist_through_first < min_dist[v] {
				vertex_queue.remove(&(NonNan(min_dist[v]), v as VertexTyp));

				min_dist[v] = dist_through_first;
				previous[v] = u;
				vertex_queue.insert((NonNan(min_dist[v]), v as VertexTyp));
			}
		}
	}
}

fn dijkstra_shortest_path(vertex: VertexTyp, previous: &Vec<VertexTyp>) -> VecDeque<VertexTyp> {
	let mut done = false;
	let mut path: VecDeque<VertexTyp> = VecDeque::new();
	let mut vertex = vertex;
	while !done {
		path.push_front(vertex);
		vertex = previous[vertex as usize];
		if vertex == -1 {
			done = true;
		}
	}
	path
}